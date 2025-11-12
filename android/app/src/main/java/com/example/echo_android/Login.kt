package com.example.echo_android

import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import com.example.rocketreserver.LoginMutation
import kotlinx.coroutines.launch
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue

@Composable
fun Login(navigateBack: () -> Unit
) {
    /*
    login画面UI
     */
    // todo: home画面に戻るのでいいのかな？
    val scope = rememberCoroutineScope()
    var username by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }

    Column {
        TextField(
            value = username,
            onValueChange = {username = it},
            label = {Text("username")}
        )
        TextField(
            value = password,
            onValueChange = {password = it},
            label = {Text("password")}
        )
        Button(
            onClick = {
                scope.launch {
                    val success = login(username, password)
                    if (success) {
                        Log.w("com.example.echo_android.Login", "Login successful")
                        navigateBack()
                    }
                }
            },
        ) {
            Text(text = "login")
        }
    }
}

private suspend fun login(username: String, password: String): Boolean {
    Log.d("Login", "Starting login for user: $username")

    val response = apolloClient.mutation(LoginMutation(username = username, password = password)).execute()
    return when {
        response.exception != null -> {
            Log.w("com.example.echo_android.Login", "Failed to com.example.echo_android.login", response.exception)
            false
        }

        response.hasErrors() -> {
            Log.w("com.example.echo_android.Login", "Failed to com.example.echo_android.login: ${response.errors?.get(0)?.message}")
            false
        }

        response.data?.login == null -> {
            Log.w("com.example.echo_android.Login", "Failed to com.example.echo_android.login: no com.example.echo_android.login data returned by the backend")
            false
        }

        else -> {
            val token = response.data!!.login.accessToken
            Log.w("com.example.echo_android.Login", "Setting token: $token")
            // todo: tokenを保存
            TokenRepository.setToken(token)
            val saved = TokenRepository.getToken()
            Log.d("Login", "Token saved verification: ${if (saved != null) "OK" else "FAILED"}")
            true
        }
    }
}