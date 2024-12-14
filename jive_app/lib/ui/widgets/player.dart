import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class Player extends ConsumerWidget {
  const Player({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Container(height: 12),
        Container(
            width: MediaQuery.of(context).size.width * 0.6,
            height: MediaQuery.of(context).size.width * 0.6,
            decoration: BoxDecoration(
              color: Theme.of(context).cardColor,
              borderRadius: BorderRadius.circular(12),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.2),
                  spreadRadius: 2,
                  blurRadius: 8,
                  offset: const Offset(0, 4),
                ),
              ],
            )),
        const SizedBox(height: 20),
        const Text(
          'Song Name',
          style: TextStyle(
            fontSize: 18,
            fontWeight: FontWeight.bold,
          ),
        ),
        const SizedBox(height: 16),
        SizedBox(
          width: MediaQuery.of(context).size.width * 0.8,
          child: SliderTheme(
            data: SliderTheme.of(context).copyWith(
              trackHeight: 4.0,
              thumbShape: const RoundSliderThumbShape(enabledThumbRadius: 6.0),
              overlayShape: const RoundSliderOverlayShape(overlayRadius: 14.0),
            ),
            child: Slider(
              value: 0.0,
              onChanged: (value) {},
              // activeColor: Theme.of(context).primaryColor,
              // inactiveColor: const Color(0xFFCCCCCC),
            ),
          ),
        ),
        Text('0:00 / 3:14',
            style: TextStyle(
              fontSize: 12,
              // color: const Color.fromARGB(0, 255, 255, 255),
            )),
        const SizedBox(height: 16),
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            IconButton(
              iconSize: 48,
              icon: const Icon(Icons.skip_previous),
              onPressed: () {},
            ),
            const SizedBox(width: 32),
            IconButton(
              iconSize: 48,
              icon: const Icon(Icons.play_circle_filled),
              onPressed: () {},
            ),
            const SizedBox(width: 32),
            IconButton(
              iconSize: 48,
              icon: const Icon(Icons.skip_next),
              onPressed: () {},
            ),
          ],
        ),
      ],
    );
  }
}
