package com.example.echo_android.repository

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey

object TokenRepository {
    private const val KEY_TOKEN = "TOKEN"
    private lateinit var preferences: SharedPreferences

    fun init(context: Context) {
        Log.d("TokenRepository", "Initializing...")

        val masterKey = MasterKey.Builder(context, MasterKey.DEFAULT_MASTER_KEY_ALIAS)
            .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
            .build()

        preferences = EncryptedSharedPreferences.create(
            context,
            "secret_shared_prefs",
            masterKey,
            EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
            EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM,
        )

    }

    fun getAccessToken(): String? {
        return preferences.getString(KEY_TOKEN, null)
    }

    fun setToken(token: String) {
        Log.d("TokenRepository", "setToken called with: ${token.take(10)}...")
        preferences.edit().apply() {
            putString(KEY_TOKEN, token)
            apply()
        }
        val saved = preferences.getString(KEY_TOKEN, null)
        Log.d("TokenRepository", "Token saved: ${if (saved != null) "Success" else "Failed"}")
    }

    fun removeToken() {
        preferences.edit().apply() {
            remove(KEY_TOKEN)
            apply()
        }
    }
}