package com.example.echo_android

import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import com.example.rocketreserver.LoginMutation
import kotlinx.coroutines.launch

@Composable
fun Login(navigateBack: () -> Unit) {
    /*
    login画面UI
     */
    val scope = rememberCoroutineScope()

    Column {
        TextField(value = "", onValueChange = {})
        TextField(value = "", onValueChange = {})
        Button(
            onClick = {
                scope.launch {
                    val success = login("pom", "mmmmmmm")
                    if (success) {
                        navigateBack()
                    }
                }
            },
        ) {
            Text(text = "com.example.echo_android.login")
        }
    }
}

private suspend fun login(username: String, password: String): Boolean {
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
            true
        }
    }
}