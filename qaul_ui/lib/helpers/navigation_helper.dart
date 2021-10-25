import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:qaul_ui/screens/create_account_screen.dart';
import 'package:qaul_ui/screens/home_screen.dart';

import '../screens/splash_screen.dart';

class NavigationHelper {
  static const initial = '/';
  static const createAccount = '/createAccount';
  static const home = '/home';

  static Route<T> _buildRoute<T>(
          final RouteSettings settings, final WidgetBuilder page) =>
      CupertinoPageRoute(builder: page, settings: settings);

  static Route<dynamic> onGenerateRoute(final RouteSettings _settings) {
    Widget routeWidget = const SizedBox.shrink();
    switch (_settings.name) {
      case initial:
        routeWidget = WillPopScope(
            onWillPop: () async => false, child: const SplashScreen());
        break;
      case createAccount:
        routeWidget = WillPopScope(
            onWillPop: () async => false, child: const CreateAccountScreen());
        break;
      case home:
        routeWidget = WillPopScope(
            onWillPop: () async => false, child: const HomeScreen());
        break;
      default:
        throw ArgumentError.value(_settings.name, 'Route name',
            'Handle this route in NavigationHelper.');
    }

    return _buildRoute(_settings, (context) => routeWidget);
  }
}
