# Source Engine Protocol

Since I couldn't find any decent documentation of the Source engine protocol online, I decided to make my own unofficial documentation. Hopefully everything here will continue to improve over time. ~Breadpudding

# Types

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Data_Types)

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

# Connectionless Messages

There are two header formats used. The first is the simple format which is used for sending short messages. Since the Source engine is limited to 1400 bytes per message, a fragmented format is available to send a message in multiple chunks.

### Simple Format

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Simple_Response_Format)

| Name  | Type   | Value                                 |
| ----- | ------ | ------------------------------------- |
| Magic | `long` | `-1`                                  |
| Type  | `byte` | The type of data that follows(if any) |

### Fragmented Format

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Multi-packet_Response_Format)

| Name   | Type    | Value                                                 |
| ------ | ------- | ----------------------------------------------------- |
| Magic  | `long`  | `-2`                                                  |
| ID     | `long`  | *See Below*                                           |
| Total  | `byte`  | The number of packets the message has been split into |
| Number | `byte`  | The packet number in the sequence(Starts at 0)        |
| Size   | `short` | The maximum size of each chunk(Usually 1248)          |
| Type   | `byte`  | The type of data that follows(if any)                 |

#### ID Format

The low 31 bits of the ID can be anything. However, if the most significant bit is 1 then the payload is compressed with BZ2.

#### Dealing with Compression

Compression adds the following header data to the end of the existing header data for the first packet only. When all the fragmented parts are collected, the BZ2 algorithm can be used to decompress the data.

| Name     | Type   | Value                                       |
| -------- | ------ | ------------------------------------------- |
| Size     | `long` | The size of the decompressed data           |
| Checksum | `long` | The CRC32 checksum of the decompressed data |

## 0x41

There are two known variants of this type. The first is used to get challenges from the server, while the second is used in the process of joining the server.

### Challenge Variant

Responds to: `0x55` or `0x56`

Responds with: `0x55` or `0x56`

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Request_Format_2)

Used to send challenge numbers to clients requesting player or rule data.

| Name      | Type   | Value                                             |
| --------- | ------ | ------------------------------------------------- |
| Challenge | `long` | The challenge you want the client to respond with |

### Handshake Variant

Responds to: `0x71`

Responds with: *Unknown*

Validity: *Unconfirmed*

| Name                    | Type    | Value                                 |
| ----------------------- | ------- | ------------------------------------- |
| Magic                   | `long`  | `0x5A4F4933`                          |
| Server Challenge        | `long`  | A new challenge from the server       |
| Client Challenge        | `long`  | The same challenge from the request   |
| Authentication Protocol | `long`  | The user authentication protocol used |
| Encryption Key Size     | `short` | Should be `0`                         |
| Encryption Key          | ?       | *Unknown*                             |

*More data exists in this message but its purpose is unknown*

## 0x42

TODO

## 0x44

Responds to: `0x55`

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Response_Format_2)

This message will send information about players currently on the server.

| Name    | Type   | Value                               |
| ------- | ------ | ----------------------------------- |
| Players | `byte` | The number of players on the server |

In addition to this, one of these entries is sent per player claimed:

| Name     | Type     | Value                                 |
| -------- | -------- | ------------------------------------- |
| Index    | `byte`   | Player chunk number(Starts at 0)      |
| Name     | `string` | Player name                           |
| Score    | `long`   | Player score                          |
| Duration | `float`  | Seconds the player has been connected |

## 0x45

Responds to: `0x56`

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Response_Format_3)

This message reveals the rules of the server. The rules appear to be specific convars.

| Name       | Type    | Value                         |
| ---------- | ------- | ----------------------------- |
| Rule Count | `short` | The number of rules to follow |

In addition, the following is how each rule is encoded.

| Name  | Type     | Value                   |
| ----- | -------- | ----------------------- |
| Name  | `string` | The name of the convar  |
| Value | `string` | The value of the convar |

## 0x49

Responds to: `0x54`

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Response_Format)

This is used to respond to a query with information about a TF2 server.

| Name             | Type     | Value                                                |
| ---------------- | -------- | ---------------------------------------------------- |
| Protocol Version | `byte`   | `0x11`                                               |
| Server Name      | `string` | Whatever the server name is                          |
| Map Name         | `string` | Whatever the name of the map is                      |
| Folder Name      | `string` | `tf`                                                 |
| Game Name        | `string` | This *should* be `Team Fortress` but can be anything |
| Steam App ID     | `short`  | `440`                                                |
| Player Count     | `byte`   | The number of players in the game                    |
| Max Player Count | `byte`   | The maximum number of players the server can handle  |
| Bot Count        | `byte`   | Should be `0`, nobody is crazy enough to use bots    |
| Server Type      | `byte`   | Either `d`(Dedicated), `l`, or `p`(SourceTV Relay)   |
| Environment      | `byte`   | Either `l`(Linux), `m`/`o`(macOS), or `w`(Windows)   |
| Visibility       | `byte`   | Either `1`(Password-protected) or `0`(Public)        |
| VAC Protected    | `byte`   | Either `1`(VAC secured) or `0`(Not secured)          |
| App Version      | `string` | `5615298` (April 6, 2020)                            |
| Extra Data Flags | `byte`   | *See Below*                                          |

### Extra Data Flags

The extra data flags is a bitfield that determines any extra data that has been sent. The order of the data fields in this documentation will represent the order they will appear in the message. Flags are named after the bit they represent(`1 << n`).

#### Flag 7

| Name | Type    | Value              |
| ---- | ------- | ------------------ |
| Port | `short` | Server port number |

#### Flag 4

| Name    | Type        | Value                        |
| ------- | ----------- | ---------------------------- |
| SteamID | `long long` | 64-bit SteamID of the Server |

#### Flag 6

| Name | Type     | Value                         |
| ---- | -------- | ----------------------------- |
| Port | `short`  | Spectator port for SourceTV   |
| Name | `string` | Spectator server for SourceTV |

#### Flag 5

| Name     | Type     | Value                  |
| -------- | -------- | ---------------------- |
| Keywords | `string` | The value of `sv_tags` |

#### Flag 0

| Name          | Type        | Value |
| ------------- | ----------- | ----- |
| 64-bit App ID | `long long` | `440` |

## 0x54

Responds with: `0x49`

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Request_Format)

This is used to query information about a TF2 server.

| Name    | Type     | Value                 |
| ------- | -------- | --------------------- |
| Payload | `string` | `Source Engine Query` |

## 0x55

Responds to: `0x41`

Responds with: `0x41` or `0x44`

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Request_Format_2)

This is used to request and respond with challenges to receive player data from the server.

| Name      | Type   | Value                                              |
| --------- | ------ | -------------------------------------------------- |
| Challenge | `long` | Either the challenge number or `-1` to request one |

## 0x56

Responds to: `0x41`

Responds with: `0x41` or `0x45`

Validity: [Confirmed](https://developer.valvesoftware.com/wiki/Server_queries#Request_Format_3)

This is used to request and respond with challenges to receive rule data from the server.

| Name      | Type   | Value                                              |
| --------- | ------ | -------------------------------------------------- |
| Challenge | `long` | Either the challenge number or `-1` to request one |

## 0x6B

TODO

## 0x71

Responds with: `0x41`

Validity: *Unconfirmed*

Used to begin the process of joining a server.

| Name      | Type     | Value        |
| --------- | -------- | ------------ |
| Challenge | `long`   | *Anything*   |
| Payload   | `string` | `0000000000` |

# Connection-based Messages

Validity: *Unconfirmed*

| Name        | Type   | Value                                                       |
| ----------- | ------ | ----------------------------------------------------------- |
| Message ID  | `long` | *See Below*                                                 |
| Response ID | `long` | The message number this message is responding to(0 if none) |
| Type        | `byte` | The message type                                            |

### Message IDs

Message IDs start with 1 and increment over time. It's very important the the message IDs created by the client are different from the message IDs created by the server. For example, the third message the server sends to a specific client will always be `3` and the second message a client sends to the server will always be `2`.

## 0x20

TODO

## 0x21

TODO

## 0xA1

TODO
