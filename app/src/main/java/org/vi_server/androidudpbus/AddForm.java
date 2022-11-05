package org.vi_server.androidudpbus;

import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.Switch;
import android.widget.TableRow;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

public class AddForm extends Activity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.port_form);

        Context ctx = this;
        {
            Button b = findViewById(R.id.finishAdding);
            b.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    Intent previousScreen = new Intent(getApplicationContext(), MainActivity.class);
                    String s;
                    try {
                        s = getJson();
                    } catch (Exception e) {
                        s = "Error: " + e.toString();
                    }
                    previousScreen.putExtra("t",s);
                    AddForm.this.setResult(0, previousScreen);
                    AddForm.this.finish();
                }
            });
        }

        {
            Switch b = findViewById(R.id.sendtoSpecific);
            b.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    Switch b = findViewById(R.id.sendtoSpecific);
                    TableRow r = findViewById(R.id.sendtoPeerRow);
                    r.setVisibility(b.isChecked() ? View.VISIBLE : View.GONE);
                    TableRow r2 = findViewById(R.id.allowBroadcastRow);
                    r2.setVisibility(b.isChecked() ? View.VISIBLE : View.GONE);
                }
            });
        }

        {
            Switch b = findViewById(R.id.multicastV4);
            b.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    Switch b = findViewById(R.id.multicastV4);
                    TableRow r1 = findViewById(R.id.mc4Row1);
                    r1.setVisibility(b.isChecked() ? View.VISIBLE : View.GONE);
                    TableRow r2 = findViewById(R.id.mc4Row2);
                    r2.setVisibility(b.isChecked() ? View.VISIBLE : View.GONE);
                    TableRow r3 = findViewById(R.id.mc4Row3);
                    r3.setVisibility(b.isChecked() ? View.VISIBLE : View.GONE);
                }
            });
        }

        {
            Switch b = findViewById(R.id.multicastV6);
            b.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    Switch b = findViewById(R.id.multicastV6);
                    TableRow r1 = findViewById(R.id.mc6Row1);
                    r1.setVisibility(b.isChecked() ? View.VISIBLE : View.GONE);
                    TableRow r2 = findViewById(R.id.mc6Row2);
                    r2.setVisibility(b.isChecked() ? View.VISIBLE : View.GONE);
                }
            });
        }

        {
            Switch b = findViewById(R.id.reply);
            b.setOnClickListener(new View.OnClickListener() {
                @Override
                public void onClick(View view) {
                    Switch b = findViewById(R.id.reply);
                    TableRow r1 = findViewById(R.id.replyRow1);
                    r1.setVisibility(b.isChecked() ? View.VISIBLE : View.GONE);
                    TableRow r2 = findViewById(R.id.replyRow2);
                    r2.setVisibility(b.isChecked() ? View.VISIBLE : View.GONE);
                }
            });
        }
    }

    protected String getJson() throws JSONException  {
        JSONObject o = new JSONObject();
        {
            EditText t = findViewById(R.id.port);
            o.put("port", Integer.decode(t.getText().toString()));
        }
        {
            EditText t = findViewById(R.id.bindIp);
            o.put("ip", t.getText().toString());
        }
        {
            Switch b = findViewById(R.id.reply);
            if (b.isChecked()) {
                {
                    EditText t = findViewById(R.id.sendtoLastN);
                    String s = t.getText().toString();
                    int n = 1;
                    if (!s.isEmpty()) {
                        n = Integer.decode(s);
                    }
                    o.put("last_n", n);
                }
                {
                    EditText t = findViewById(R.id.forgetMs);
                    String s = t.getText().toString();
                    if (!s.isEmpty()) {
                        o.put("forget_ms", Integer.decode(t.getText().toString()));
                    }
                }
            }
        }
        {
            Switch b = findViewById(R.id.sendtoSpecific);
            if (b.isChecked()) {
                JSONArray a = new JSONArray();
                EditText t = findViewById(R.id.specificPeer);
                String s = t.getText().toString();
                a.put(s);
                o.put("sendto", a);
            }
        }
        {
            Switch b = findViewById(R.id.allowBroadcast);
            if (b.isChecked()) {
                o.put("broadcast", true);
            }
        }
        {
            Switch b = findViewById(R.id.multicastV4);
            if (b.isChecked()) {
                EditText t = findViewById(R.id.multicastGroup4);
                String s = t.getText().toString();
                o.put("multicast_addr",s);

                EditText t2 = findViewById(R.id.multicastInterface4);
                String s2 = t2.getText().toString();
                o.put("multicast_ifaddr",s2);

                EditText t3 = findViewById(R.id.multicastTtl);
                String s3 = t3.getText().toString();
                if (!s3.isEmpty()) {
                    o.put("multicast_ttl",Integer.decode(s3));
                }
            }
        }
        {
            Switch b = findViewById(R.id.multicastV6);
            if (b.isChecked()) {

                EditText t = findViewById(R.id.multicastGroup6);
                String s = t.getText().toString();
                o.put("multicast6_addr",s);

                EditText t2 = findViewById(R.id.multicastInterface6);
                String s2 = t2.getText().toString();
                o.put("multicast6_ifindex",Integer.decode(s2));
            }
        }
        {
            EditText t = findViewById(R.id.ttl);
            String s = t.getText().toString();
            if (!s.isEmpty()) {
                o.put("ttl",Integer.decode(s));
            }
        }
        return o.toString();
    }
}