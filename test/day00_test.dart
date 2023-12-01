import 'package:aoc2023/day00.dart';
import 'package:test/test.dart';

void main() {
  test('day00_01a', () async {
    expect(await day00_01a(), 69795);
  });

  test('day00_01b', () async {
    expect(await day00_01b(), 208437);
  });
}
