# data-encoding

## 1.1 Simple conversion

| Decimal | Hexadecimal |
| --- | --- |
| 9 | 0x9 |
| 136 | 0x88 |
| 247 | 0xf7 |

## 1.2 CSS colors

0xff0000 = rgb(255, 0, 0)

256 * 256 * 256 = 16.777.216 colors

fun website :) https://yizzle.com/whatthehex/

```
echo -n -e '\xdc\xd2\xbf\x8d\x70\x0f\x07\x51\x31\x92\x3e\x34\x7c\x35\xb8\x6f\x38' > hellohex

echo -n `xxd -p hellohex` | wc -c # fish `` -> ()
```

17 bytes = 34 hexadecimal characters

68656c6c6f = `hello`

6 * 16 = 96

0x68 = 6 * 16 + 08 = 104 = h = 0b0110 0b1000
0x65 = 6 * 16 + 05 = 101 = e = 0b0110 0b0101
0x6c = 6 * 16 + 12 = 108 = l = 0b0110 0b1100
0x6c = 6 * 16 + 13 = 108 = l = 0b0110 0b1100
0x6f = 6 * 16 + 16 = 111 = o = 0b0110 0b1111

https://web.fe.up.pt/~ee96100/projecto/Tabela%20ascii.htm

## 2.1 Basic conversion

4 = 0b100 = 0x04
65 = 0b1000001 = 0x41
105 = 0b1101001 = 0x69
255 = 0b11111111 = 0xff

10 = 2 = 0x02
11 = 3 = 0x03
1101100 = 108 = 0x6c
1010101 = 85 = 0x55
