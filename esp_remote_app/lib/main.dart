import 'package:bluetooth_enable_fork/bluetooth_enable_fork.dart';
import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

Future<void> checkPerm() async {
  var permissions = [
    Permission.bluetoothScan,
    Permission.bluetoothConnect,
  ];
  for (var perm in permissions) {
    if (await perm.isDenied) {
      await perm.request();
    }
    if (await perm.status.isPermanentlyDenied) {
      await openAppSettings();
    }
  }
  var enabled = await BluetoothEnable.enableBluetooth;
  if (enabled == "false") {
    print("enable bluetooth first");
  }
}

void main() async {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // Try running your application with "flutter run". You'll see the
        // application has a blue toolbar. Then, without quitting the app, try
        // changing the primarySwatch below to Colors.green and then invoke
        // "hot reload" (press "r" in the console where you ran "flutter run",
        // or simply save your changes to "hot reload" in a Flutter IDE).
        // Notice that the counter didn't reset back to zero; the application
        // is not restarted.
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  // These futures belong to the state and are only initialized once,
  // in the initState method.
  List<BleDevice> bleDevices = [];

  @override
  void initState() {
    super.initState();
    async_init();
  }

  void async_init() async {
    api.createLogStream().listen((log) => print(
        "rust-log ${Duration(milliseconds: log.timeMillis)}: ${log.msg}"));
    await api.init();
  }

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
      appBar: AppBar(
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: Text(widget.title),
      ),
      body: Column(
        children: [
          TextButton(
            child: Text("test logging"),
            onPressed: () => api.logTest(),
          ),
          TextButton(
            child: Text("check permissions"),
            onPressed: () => checkPerm(),
          ),
          TextButton(
            child: Text("test discover"),
            onPressed: () {
              api.bleDiscover(timeout: 5000).listen((devices) => setState(() {
                    bleDevices = devices;
                  }));
            },
          ),
          Expanded(
            child: ListView.builder(
              itemCount: bleDevices.length,
              itemBuilder: (ctx, idx) => SizedBox(
                width: 100,
                height: 20,
                child: TextButton(
                    onPressed: () async {
                      await api.bleConnect(id: bleDevices[idx].address);
                    },
                    child: Text(
                        "${bleDevices[idx].name} (${bleDevices[idx].address})")),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
