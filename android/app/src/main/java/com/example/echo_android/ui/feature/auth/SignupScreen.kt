package com.example.echo_android.ui.feature.auth

import android.util.Log
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.hilt.navigation.compose.hiltViewModel
import com.example.echo_android.repository.TokenRepository
import com.example.echo_android.network.apolloClient
import com.example.rocketreserver.SignupMutation

@Composable
fun SignupScreen(
    navigateBack: () -> Unit,
    onSecondaryClick: () -> Unit,
    viewModel: SignupViewModel = hiltViewModel()
) {
    val success by viewModel.state.collectAsState()

    LaunchedEffect(success) {
        if (success) navigateBack()
    }

    AuthScreen(
        title = "Signup",
        buttonText = "登録",
        onAuthClick = { username, password ->
            viewModel.signup(username, password)
            success
        },
        onSecondaryClick = { onSecondaryClick() },
        secondaryText = "ログインはこちら"
    )
}

private suspend fun signup(username: String, password: String): Boolean {
    Log.d("Signup", "Starting signup for user: $username")

    val response = apolloClient.mutation(SignupMutation(username = username, password = password)).execute()
    return when {
        response.exception != null -> {
            Log.w("com.example.echo_android.Signup", "Failed to com.example.echo_android.ui.feature.auth.signup", response.exception)
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