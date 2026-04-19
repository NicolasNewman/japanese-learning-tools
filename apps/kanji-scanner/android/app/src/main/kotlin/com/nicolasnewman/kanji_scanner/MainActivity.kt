package com.nicolasnewman.kanji_scanner

import androidx.annotation.NonNull
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import androidx.core.app.ActivityCompat
import com.ichi2.anki.api.AddContentApi.READ_WRITE_PERMISSION

class MainActivity : FlutterActivity() {
    private val CHANNEL = "com.nicolasnewman.kanji_scanner/anki_read"

    private val PERMISSION_REQUEST_CODE = 123

    // private val mAnkiDroid = AnkiDroidHelper(this);
//    private val mAnkiDroid by lazy { AnkiDroidHelper(this) }
//    private val mNativeApi by lazy { NativeApiImplementation(mAnkiDroid) }

    override fun configureFlutterEngine(@NonNull flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
//        NativeApi.setUp(flutterEngine.dartExecutor.binaryMessenger, mNativeApi)
//
//        if (!mAnkiDroid.hasPermission()) {
//            ActivityCompat.requestPermissions(
//                this,
//                arrayOf(READ_WRITE_PERMISSION),
//                PERMISSION_REQUEST_CODE
//            )
//        }
    }
}