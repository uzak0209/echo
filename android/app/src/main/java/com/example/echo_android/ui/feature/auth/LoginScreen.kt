package com.example.echo_android.ui.feature.auth

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.hilt.navigation.compose.hiltViewModel

@Composable
fun LoginScreen(
    navigateBack: () -> Unit,
    onSecondaryClick: () -> Unit,
    viewModel: LoginViewModel = hiltViewModel()
) {
    val authResult by viewModel.state.collectAsState()
    var errorMessage by remember { mutableStateOf<String?>(null) }

    LaunchedEffect(authResult) {
        when (authResult) {
            is LoginViewModel.AuthResult.Success -> navigateBack()
            is LoginViewModel.AuthResult.Error -> {
                errorMessage = (authResult as LoginViewModel.AuthResult.Error).message
            }
            else -> {}
        }
    }

    AuthScreen(
        title = "Login",
        buttonText = "ログイン",
        onAuthClick = { username, password ->
            errorMessage = null
            viewModel.login(username, password)
            true
        },
        onSecondaryClick = { onSecondaryClick()},
        secondaryText = "アカウント作成",
        externalError = errorMessage
    )
}