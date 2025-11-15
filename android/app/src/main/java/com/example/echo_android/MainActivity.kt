package com.example.echo_android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeDrawing
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.example.echo_android.network.ApolloClientFactory
import com.example.echo_android.ui.feature.MainScreen
import com.example.echo_android.ui.feature.auth.SignupScreen
import com.example.echo_android.ui.navigation.NavigationDestinations
import com.example.echo_android.ui.theme.EchoandroidTheme
import dagger.hilt.android.AndroidEntryPoint

@AndroidEntryPoint
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        ApolloClientFactory.initialize(this)
        enableEdgeToEdge()
        setContent {
            EchoandroidTheme {
                Scaffold(modifier = Modifier.fillMaxSize().windowInsetsPadding(WindowInsets.safeDrawing)) { innerPadding ->
                    MainNavHost()
                }
            }
        }
    }
}

@Composable
private fun MainNavHost() {
    val navController = rememberNavController()
    NavHost(navController, startDestination = NavigationDestinations.LOGIN) {
        composable(route = NavigationDestinations.LOGIN) {
            SignupScreen(
                navigateBack = {
                    navController.navigate(NavigationDestinations.HOME)
                },
            )
        }

        composable(route = NavigationDestinations.HOME) {
            MainScreen()
        }
    }
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    EchoandroidTheme {
    }
}