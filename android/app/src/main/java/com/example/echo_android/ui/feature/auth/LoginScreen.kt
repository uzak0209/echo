package com.example.echo_android.ui.feature.auth

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.hilt.navigation.compose.hiltViewModel

@Composable
fun LoginScreen(
    navigateBack: () -> Unit,
    onSecondaryClick: () -> Unit,
    viewModel: LoginViewModel = hiltViewModel()
) {
    val success by viewModel.state.collectAsState()

    LaunchedEffect(success) {
        if (success) navigateBack()
    }

    AuthScreen(
        title = "Login",
        buttonText = "ログイン",
        onAuthClick = { username, password ->
            viewModel.login(username, password)
            true
        },
        onSecondaryClick = { onSecondaryClick()},
        secondaryText = "アカウント作成"
    )
}