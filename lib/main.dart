import 'package:flutter/material.dart';
import 'package:serenity_dart_compat/src/rust/api/simple.dart';
import 'package:serenity_dart_compat/src/rust/frb_generated.dart';
import 'package:serenity_dart_compat/src/rust/lib.dart';
import 'package:serenity_dart_compat/src/rust/third_party/serenity/client.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  void initState() {
    print(ClientBuilder);
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
            'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`',
          ),
        ),
      ),
    );
  }
}
