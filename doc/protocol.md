# Source Engine Protocol

Since I couldn't find any decent documentation of the Source engine protocol online, I decided to make my own unofficial documentation. Hopefully everything here will continue to improve over time. ~Breadpudding

# Protocol Details

The TF2 server uses a UDP socket for communication with a default port of 27015. UDP messages are no larger than 1400 bytes.

# Types

All types larger than a byte is stored in the little endian format.

| Valve       | C           | Rust        |
| ----------- | ----------- | ----------- |
| `byte`      | `uint8_t`   | `u8`        |
| `short`     | `int16_t`   | `i16`       |
| `long`      | `int32_t`   | `i32`       |
| `float`     | `float`     | `f32`       |
| `long long` | `uint64_t`  | `u64`       |
| `string`    | *See Below* | *See Below* |

## Strings

All strings handled by the Source engine are zero-terminated UTF-8 strings.

# List of Source messages

## A2A_ACK

Type: `j`

TODO

## M2A_ACTIVEMODS

Type: `y`

TODO

## M2A_ACTIVEMODS3

Type: `P`

TODO

## S2C_AUTHCHALLENGE1

Type: `4`

TODO

## S2C_AUTHCHALLENGE2

Type: `5`

TODO

## S2C_AUTHCOMPLETE

Type: `6`

TODO

## C2S_AUTHCONNECT

Type: `7`

TODO

## C2S_AUTHREQUEST1

Type: `3`

TODO

## S2C_CHALLENGE

Type: `A`

TODO

## C2M_CHECKMD5

Type: `M`

TODO

## C2S_CONNECT

Type: `k`

TODO

## S2C_CONNECTION

Type: `B`

TODO

## S2C_CONNREJECT

Type: `9`

TODO

## A2A_CUSTOM

Type: `t`

TODO

## A2M_GET_MOTD

Type: `g`

TODO

## A2M_GET_SERVERS

Type: `c`

TODO

## A2M_GET_SERVERS_BATCH

Type: `e`

TODO

## A2M_GET_SERVERS_BATCH2

Type: `1`

TODO

## A2M_GETACTIVEMODS

Type: `x`

TODO

## A2M_GETACTIVEMODS2

Type: `2`

TODO

## A2M_GETACTIVEMODS3

Type: `Q`

TODO

## A2S_GETCHALLENGE

Type: `q`

TODO

## S2M_GETFILE

Type: `J`

TODO

## A2M_GETMASTERSERVERS

Type: `v`

TODO

## A2S_INFO

Type: `T`

TODO

## S2A_INFO_DETAILED

Type: `m`

TODO

## S2A_INFO_GOLDSRC

Type: `m`

TODO

## S2A_INFO_SRC

Type: `I`

TODO

## M2C_ISVALIDMD5

Type: `N`

TODO

## S2A_LOGSTRING

Type: `R`

TODO

## S2A_LOGSTRING2

Type: `S`

TODO

## M2A_MASTERSERVERS

Type: `w`

TODO

## C2C_MOD

Type: `X`

TODO

## M2A_MOTD

Type: `h`

TODO

## M2M_MSG

Type: `z`

TODO

## A2A_PING

Type: `i`

TODO

## A2S_PING2

Type: `Y`

TODO

## S2A_PING2REPLY

Type: `Z`

TODO

## A2S_PLAYER

Type: `U`

TODO

## A2A_PRINT

Type: `l`

TODO

## A2S_RCON

Type: `r`

TODO

## S2C_REDIRECT

Type: `L`

TODO

## A2S_RULES

Type: `V`

TODO

## M2S_SENDFILE

Type: `K`

TODO

## A2S_SERVERQUERY_GETCHALLENGE

Type: `W`

TODO

## M2A_SERVER_BATCH

Type: `f`

TODO

## M2A_SERVERS

Type: `d`

TODO
