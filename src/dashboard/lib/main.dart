import 'package:flutter/material.dart';
import 'package:dashboard/bridge_generated.dart';
import 'dart:async';
import 'dart:ffi';

const path = 'dlib/libadapter.so';
late final dylib = DynamicLibrary.open(path);
late final ttb2Adapter = Adapter(dylib);

void main() {
  _spawn();
  runApp(const MyApp());
}

Future<void> _spawn() async {
  final a = ttb2Adapter.spawnAdapter();
  await for (final v in a) {
    debugPrint(v.toString());
  }
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headline4,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }

  void _incrementCounter() {
    setState(() {
      _counter++;
      _print();
    });
  }

  Future<void> _print() async {
    await ttb2Adapter.availableTutlebots().then((v) {
      v.forEach(debugPrint);
    }).catchError((e) {
      debugPrint("Error: " + e.toString());
    });

    await ttb2Adapter.sendToAdapter();
  }
}
