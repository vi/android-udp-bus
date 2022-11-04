package org.vi_server.androidudpbus;


import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.preference.Preference;
import android.text.Editable;
import android.text.TextWatcher;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;

import org.json.JSONArray;
import org.json.JSONObject;
import org.w3c.dom.Text;

import java.util.Timer;
import java.util.TimerTask;

public class MainActivity extends Activity {

    Timer timer = new Timer();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        Context ctx = this;
        setContentView(R.layout.activity_main);
        {
            Button b = findViewById(R.id.addPort);
            b.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    Intent intent = new Intent(ctx, AddForm.class);
                    startActivityForResult(intent, 0);
                }
            });
        }

        {
            Button b = findViewById(R.id.start);
            b.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    Intent intent = new Intent(ctx, Serv.class);

                    EditText t = findViewById(R.id.configEditor);
                    String config = t.getText().toString();

                    intent.putExtra("config", config);
                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                        ctx.startForegroundService(intent);
                    } else {
                        ctx.startService(intent);
                    }
                }
            });
        }

        {
            Button b = findViewById(R.id.stop);
            b.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    Intent intent = new Intent(ctx, Serv.class);
                    ctx.stopService(intent);
                }
            });
        }

        Handler h = new Handler();
        timer.scheduleAtFixedRate(new TimerTask() {
            @Override
            public void run() {
                String s = Serv.startupError;
                if (s == null) {
                    s = "idle";
                }
                synchronized (Serv.instance) {
                    if (Serv.instance.self != 0) {
                        s = Native.getStats(Serv.instance.self, Native.STATS_LONG);
                    }
                }
                final String ss = s;
                h.post(new Runnable() {
                    @Override
                    public void run() {
                        EditText t = findViewById(R.id.statusViewer);
                        t.setText(ss);
                    }
                });
            }
        }, 0, 250);

        {
            EditText c = findViewById(R.id.configEditor);
            c.addTextChangedListener(new TextWatcher() {
                @Override
                public void beforeTextChanged(CharSequence charSequence, int i, int i1, int i2) {

                }

                @Override
                public void onTextChanged(CharSequence charSequence, int i, int i1, int i2) {

                }

                @Override
                public void afterTextChanged(Editable editable) {
                    String s = Native.checkConfig(editable.toString());
                    if (s == null) {
                        s="OK";
                    }
                    TextView v = findViewById(R.id.configStatus);
                    v.setText(s);
                }
            });
        }

        {
            Button b = findViewById(R.id.save);
            b.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    EditText t = findViewById(R.id.configEditor);
                    String config = t.getText().toString();
                    SharedPreferences.Editor e = getPreferences(MODE_PRIVATE).edit();
                    e.putString("config", config);
                    e.apply();
                }
            });
        }

        {
            String s = getPreferences(MODE_PRIVATE).getString("config", null);
            if (s != null) {
                EditText t = findViewById(R.id.configEditor);
                t.setText(s);
            }
        }
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        EditText t = findViewById(R.id.configEditor);
        if (data != null) {
            String s = data.getStringExtra("t");
            JSONArray a;
            if (!t.getText().toString().isEmpty()) {
                try {
                    a = new JSONArray((t.getText().toString()));

                    if (a.length() == 0) {
                        a.put(new JSONArray());
                    }
                    JSONArray aa = (JSONArray) a.get(0);

                    JSONObject o = new JSONObject(s);
                    aa.put(o);
                    t.setText(a.toString(2));
                } catch (Exception e) {
                    TextView v = findViewById(R.id.configStatus);
                    v.setText(e.toString());

                    t.append("\n");
                    t.append(s);
                    return;
                }
            }
        }
    }

    @Override
    protected void onDestroy() {
        timer.cancel();
        timer = null;
        super.onDestroy();
    }
}