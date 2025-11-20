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
            true
        },
        onSecondaryClick = { onSecondaryClick() },
        secondaryText = "ログインはこちら"
    )
}