package com.example.echo_android

import android.util.Log
import androidx.compose.runtime.Composable
import com.example.echo_android.ui.AuthScreen
import com.example.rocketreserver.SignupMutation

@Composable
fun Signup(
    navigateBack: () -> Unit
) {
    AuthScreen(
        title = "Signup",
        buttonText = "登録",
        onAuthClick = { username, password ->
            val success = signup(username, password)
            if (success) navigateBack()
            success
        },
        onSecondaryClick = { /* todo: ログイン画面へ戻る */ },
        secondaryText = "ログインはこちら"
    )
}

private suspend fun signup(username: String, password: String): Boolean {
    Log.d("Signup", "Starting signup for user: $username")

    val response = apolloClient.mutation(SignupMutation(username = username, password = password)).execute()
    return when {
        response.exception != null -> {
            Log.w("com.example.echo_android.Signup", "Failed to com.example.echo_android.signup", response.exception)
            false
        }

        response.hasErrors() -> {
            Log.w("com.example.echo_android.Signup", "Failed to com.example.echo_android.sign: ${response.errors?.get(0)?.message}")
            false
        }

        else -> {
            val token = response.data!!.signup.accessToken
            Log.w("com.example.echo_android.Signup", "Setting token: $token")
            TokenRepository.setToken(token)
            val saved = TokenRepository.getToken()
            Log.d("Signup", "Token saved verification: ${if (saved != null) "OK" else "FAILED"}")
            true
        }
    }
}