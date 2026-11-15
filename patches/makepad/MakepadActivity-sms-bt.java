    // Wadhu SMS Gateway: Classic RFCOMM SPP server for offline SMS jobs.
    // Inserted by scripts/apply-android-activity-bridge.sh — do not edit in vendor.
    // Marker: wadhuSmsBtPieceEnds
    // SPP UUID 00001101-0000-1000-8000-00805F9B34FB — see apps/sms-gateway/docs/BLUETOOTH.md

    private static final java.util.UUID WADHU_SMS_SPP_UUID =
            java.util.UUID.fromString("00001101-0000-1000-8000-00805F9B34FB");
    private static final String WADHU_SMS_BT_NAME = "Wadhu SMS Gateway";
    private volatile android.bluetooth.BluetoothServerSocket wadhuBtServer;
    private volatile android.bluetooth.BluetoothSocket wadhuBtClient;
    private volatile Thread wadhuBtAcceptThread;
    private volatile boolean wadhuBtRunning;
    private volatile String wadhuBtDetail = "off";

    public synchronized String btRfcommStart() {
        try {
            if (android.os.Build.VERSION.SDK_INT >= 31) {
                java.util.ArrayList<String> need = new java.util.ArrayList<>();
                if (checkSelfPermission(android.Manifest.permission.BLUETOOTH_CONNECT)
                        != android.content.pm.PackageManager.PERMISSION_GRANTED) {
                    need.add(android.Manifest.permission.BLUETOOTH_CONNECT);
                }
                if (checkSelfPermission(android.Manifest.permission.BLUETOOTH_ADVERTISE)
                        != android.content.pm.PackageManager.PERMISSION_GRANTED) {
                    need.add(android.Manifest.permission.BLUETOOTH_ADVERTISE);
                }
                if (checkSelfPermission(android.Manifest.permission.BLUETOOTH_SCAN)
                        != android.content.pm.PackageManager.PERMISSION_GRANTED) {
                    need.add(android.Manifest.permission.BLUETOOTH_SCAN);
                }
                if (!need.isEmpty()) {
                    try {
                        requestPermissions(need.toArray(new String[0]), 71010);
                    } catch (Exception ignore) {}
                    wadhuBtDetail = "permission_needed";
                    return wadhuBtDetail;
                }
            }
            android.bluetooth.BluetoothManager btMgr =
                    (android.bluetooth.BluetoothManager) getSystemService(BLUETOOTH_SERVICE);
            android.bluetooth.BluetoothAdapter adapter =
                    btMgr != null ? btMgr.getAdapter() : null;
            if (adapter == null) {
                wadhuBtDetail = "no_adapter";
                return wadhuBtDetail;
            }
            if (!adapter.isEnabled()) {
                wadhuBtDetail = "adapter_off";
                return wadhuBtDetail;
            }
            if (wadhuBtRunning && wadhuBtServer != null) {
                wadhuBtDetail = "listening";
                return wadhuBtDetail;
            }
            btRfcommStopUnlocked();
            try {
                adapter.setName(WADHU_SMS_BT_NAME);
            } catch (SecurityException se) {
                Log.w(LOG_TAG, "btRfcommStart: setName denied", se);
            }
            android.bluetooth.BluetoothServerSocket server =
                    adapter.listenUsingRfcommWithServiceRecord(WADHU_SMS_BT_NAME, WADHU_SMS_SPP_UUID);
            wadhuBtServer = server;
            wadhuBtRunning = true;
            wadhuBtDetail = "listening";
            wadhuBtAcceptThread = new Thread(() -> {
                while (wadhuBtRunning) {
                    try {
                        android.bluetooth.BluetoothSocket sock = server.accept();
                        if (sock == null) {
                            continue;
                        }
                        synchronized (MakepadActivity.this) {
                            if (wadhuBtClient != null) {
                                try { wadhuBtClient.close(); } catch (Exception ignore) {}
                            }
                            wadhuBtClient = sock;
                            wadhuBtDetail = "connected";
                            MakepadActivity.this.notifyAll();
                        }
                    } catch (java.io.IOException e) {
                        if (wadhuBtRunning) {
                            Log.e(LOG_TAG, "btRfcomm accept failed", e);
                            wadhuBtDetail = "accept_error";
                        }
                        break;
                    }
                }
            }, "wadhu-sms-bt-accept");
            wadhuBtAcceptThread.setDaemon(true);
            wadhuBtAcceptThread.start();
            return wadhuBtDetail;
        } catch (SecurityException se) {
            Log.e(LOG_TAG, "btRfcommStart permission", se);
            wadhuBtDetail = "permission_needed";
            return wadhuBtDetail;
        } catch (Exception e) {
            Log.e(LOG_TAG, "btRfcommStart failed", e);
            wadhuBtDetail = "error:" + e.getClass().getSimpleName();
            return wadhuBtDetail;
        }
    }

    public synchronized void btRfcommStop() {
        btRfcommStopUnlocked();
        wadhuBtDetail = "off";
    }

    private void btRfcommStopUnlocked() {
        wadhuBtRunning = false;
        try {
            if (wadhuBtServer != null) {
                wadhuBtServer.close();
            }
        } catch (Exception ignore) {}
        wadhuBtServer = null;
        try {
            if (wadhuBtClient != null) {
                wadhuBtClient.close();
            }
        } catch (Exception ignore) {}
        wadhuBtClient = null;
        wadhuBtAcceptThread = null;
    }

    public synchronized String btRfcommStatus() {
        return wadhuBtDetail == null ? "off" : wadhuBtDetail;
    }

    /** Block until a client is connected or timeoutMs elapses. */
    public synchronized boolean btRfcommWaitClient(int timeoutMs) {
        long deadline = System.currentTimeMillis() + Math.max(1, timeoutMs);
        while (wadhuBtClient == null && wadhuBtRunning) {
            long left = deadline - System.currentTimeMillis();
            if (left <= 0) {
                return false;
            }
            try {
                wait(left);
            } catch (InterruptedException ie) {
                return false;
            }
        }
        return wadhuBtClient != null;
    }

    /**
     * Read exactly n bytes from the connected client (null on failure / timeout).
     * BluetoothSocket has no setSoTimeout — enforce timeoutMs with a watchdog that
     * closes the socket so a blocked InputStream.read unblocks.
     */
    public byte[] btRfcommReadExact(int n, int timeoutMs) {
        final android.bluetooth.BluetoothSocket sock;
        synchronized (this) {
            sock = wadhuBtClient;
        }
        if (sock == null || n <= 0 || n > 65536) {
            return null;
        }
        final int waitMs = Math.max(1, timeoutMs);
        final java.util.concurrent.atomic.AtomicBoolean timedOut =
                new java.util.concurrent.atomic.AtomicBoolean(false);
        Thread watchdog = new Thread(() -> {
            try {
                Thread.sleep(waitMs);
            } catch (InterruptedException ie) {
                return;
            }
            timedOut.set(true);
            try {
                sock.close();
            } catch (Exception ignore) {}
        }, "wadhu-sms-bt-read-watchdog");
        watchdog.setDaemon(true);
        watchdog.start();
        try {
            java.io.InputStream in = sock.getInputStream();
            byte[] buf = new byte[n];
            int off = 0;
            while (off < n) {
                int r = in.read(buf, off, n - off);
                if (r < 0) {
                    btRfcommClearClientAfterIo();
                    return null;
                }
                off += r;
            }
            watchdog.interrupt();
            return buf;
        } catch (Exception e) {
            if (!timedOut.get()) {
                Log.e(LOG_TAG, "btRfcommReadExact failed", e);
            }
            btRfcommClearClientAfterIo();
            return null;
        }
    }

    private synchronized void btRfcommClearClientAfterIo() {
        try {
            if (wadhuBtClient != null) {
                wadhuBtClient.close();
            }
        } catch (Exception ignore) {}
        wadhuBtClient = null;
        if (wadhuBtRunning) {
            wadhuBtDetail = "listening";
        }
    }

    public boolean btRfcommWriteAll(byte[] data) {
        android.bluetooth.BluetoothSocket sock;
        synchronized (this) {
            sock = wadhuBtClient;
        }
        if (sock == null || data == null) {
            return false;
        }
        try {
            java.io.OutputStream out = sock.getOutputStream();
            out.write(data);
            out.flush();
            return true;
        } catch (Exception e) {
            Log.e(LOG_TAG, "btRfcommWriteAll failed", e);
            synchronized (this) {
                try { if (wadhuBtClient != null) wadhuBtClient.close(); } catch (Exception ignore) {}
                wadhuBtClient = null;
                if (wadhuBtRunning) {
                    wadhuBtDetail = "listening";
                }
            }
            return false;
        }
    }

    public synchronized void btRfcommDisconnectClient() {
        try {
            if (wadhuBtClient != null) {
                wadhuBtClient.close();
            }
        } catch (Exception ignore) {}
        wadhuBtClient = null;
        if (wadhuBtRunning) {
            wadhuBtDetail = "listening";
        }
    }
    // wadhuSmsBtPieceEnds
