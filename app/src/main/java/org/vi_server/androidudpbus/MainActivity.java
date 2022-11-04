package org.vi_server.androidudpbus;


import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;

import org.json.JSONArray;
import org.json.JSONObject;

public class MainActivity extends Activity {

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
}