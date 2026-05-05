package com.nicolasnewman.kanji_scanner

import androidx.annotation.NonNull
import androidx.core.app.ActivityCompat
import com.ichi2.anki.api.AddContentApi
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine

class MainActivity : FlutterActivity() {
    private val PERMISSION_REQUEST_CODE = 123

    private val mAnkiDroid by lazy { AnkiDroidHelper(this) }  // Use lazy initialization
    private val mNativeApi by lazy { NativeApiImplementation(mAnkiDroid) }

    override fun configureFlutterEngine(@NonNull flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
        NativeApi.setUp(flutterEngine.dartExecutor.binaryMessenger, mNativeApi)
        if (!mAnkiDroid.hasPermission()) {
            ActivityCompat.requestPermissions(
                this,
                arrayOf(AddContentApi.Companion.READ_WRITE_PERMISSION),
                PERMISSION_REQUEST_CODE
            )
        }
    }
}