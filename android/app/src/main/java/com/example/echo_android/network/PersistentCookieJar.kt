package com.example.echo_android.network

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import okhttp3.Cookie
import okhttp3.CookieJar
import okhttp3.HttpUrl

class PersistentCookieJar(context: Context) : CookieJar {
    private val preferences: SharedPreferences = context.getSharedPreferences(
        "cookie_prefs",
        Context.MODE_PRIVATE
    )

    override fun saveFromResponse(url: HttpUrl, cookies: List<Cookie>) {
        Log.d("PersistentCookieJar", "saveFromResponse called for URL: $url with ${cookies.size} cookies")
        cookies.forEach { cookie ->
            Log.d("PersistentCookieJar", "Saving cookie: ${cookie.name} = ${cookie.value.take(20)}... (domain: ${cookie.domain}, path: ${cookie.path}, httpOnly: ${cookie.httpOnly}, secure: ${cookie.secure})")
            preferences.edit()
                .putString(cookie.name, encodeCookie(cookie))
                .apply()
        }
    }

    override fun loadForRequest(url: HttpUrl): List<Cookie> {
        Log.d("PersistentCookieJar", "loadForRequest called for URL: $url")
        val cookies = mutableListOf<Cookie>()
        preferences.all.forEach { (_, value) ->
            val cookieString = value as? String
            if (cookieString != null) {
                val cookie = decodeCookie(cookieString)
                if (cookie != null) {
                    if (cookie.expiresAt < System.currentTimeMillis()) {
                        Log.d("PersistentCookieJar", "Cookie expired: ${cookie.name}")
                    } else {
                        cookies.add(cookie)
                        Log.d("PersistentCookieJar", "Loading cookie: ${cookie.name} = ${cookie.value.take(20)}... for request")
                    }
                }
            }
        }
        Log.d("PersistentCookieJar", "Loaded ${cookies.size} cookies for request")
        return cookies
    }

    fun clear() {
        preferences.edit().clear().apply()
        Log.d("PersistentCookieJar", "Cookies cleared")
    }

    private fun encodeCookie(cookie: Cookie): String {
        return "${cookie.name}|${cookie.value}|${cookie.expiresAt}|${cookie.domain}|${cookie.path}|${cookie.secure}|${cookie.httpOnly}"
    }

    private fun decodeCookie(cookieString: String): Cookie? {
        return try {
            val parts = cookieString.split("|")
            if (parts.size != 7) return null

            Cookie.Builder()
                .name(parts[0])
                .value(parts[1])
                .expiresAt(parts[2].toLong())
                .domain(parts[3])
                .path(parts[4])
                .apply {
                    if (parts[5].toBoolean()) secure()
                    if (parts[6].toBoolean()) httpOnly()
                }
                .build()
        } catch (e: Exception) {
            Log.e("PersistentCookieJar", "Failed to decode cookie: $cookieString", e)
            null
        }
    }
}
