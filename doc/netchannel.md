# Netchannel Data
## NetChannel Packet Header

| Name           | Type   | Description                              | Value       |
|----------------|--------|------------------------------------------|-------------|
| Sequence       | `long` | The sequence number                      | *Varies*    |
| Sequence ack   | `long` | The sequence number acked                | *Varies*    |
| Flags          | `byte` | A series of flags that change the header | *Varies*    |
| Checksum       | `u16`  | A checksum of the message                | *See below* |
| Reliable State | `byte` | Reliable state of 8 subchannels          | *Varies*    |

### Checksum

**TODO**: Document the checksum

### Flags

#### `PACKET_FLAG_CHOKED` = `1 << 4`

| Name   | Type   | Description                   | Value             |
|--------|--------|-------------------------------|-------------------|
| Choked | `byte` | The number of packets choked? | `Number Choked & 0xFF` |

#### `PACKET_FLAG_CHALLENGE` = `1 << 5`

| Name      | Type   | Description                               | Value    |
|-----------|--------|-------------------------------------------|----------|
| Challenge | `long` | A challenge to verify the client identity | *Varies* |

#### `PACKET_FLAG_RELIABLE` = `1 << 0`
##### Reliable Data Structure

| Name            | Type        | Description                                      | Value    |
|-----------------|-------------|--------------------------------------------------|----------|
| Bit             | `3 bits`    | The bit to flip in reliable state (1 << this)    | *Varies* |
| SubChannel Data | *See below* | Data for a subchannel (Repeats 2 times)          | *Varies* |

##### SubChannel Data (Single Block)

| Name              | Type                     | Description                                                                                     | Value                |
|-------------------|--------------------------|-------------------------------------------------------------------------------------------------|----------------------|
| Exists            | `1 bit`                  | If there is data to read for the subchannel (Determines if the rest of this structure is there) | *It is binary*       |
| Single Block?     | `1 bit`                  | If the data is in a single block or multiple                                                    | `0` means **single** |
| Compressed?       | `1 bit`                  | If the data is compressed                                                                       | *It is binary*       |
| Uncompressed size | `26 bits`                | The uncompressed size of the data. **Only read if the packet is compressed**                    | *Varies*             |
| Bytes             | `VarInt32`               | The size of the file.                                                                           | *Varies*             |
| Data              | `bytes+(1 << 8)-1 bytes` | The data being sent.                                                                            | *Varies*             |

##### SubChannel Data (Multi-Block)
###### First Block

| Name                | Type                     | Description                                                       | Value                         |
|---------------------|--------------------------|-------------------------------------------------------------------|-------------------------------|
| Single Block?       | `bit`                    | If the message is a single block                                  | `1` means **multiple blocks** |
| Start Fragment      | `26 - 8 bits`            | The start fragment of the packet                                  | *Varies*                      |
| Number of fragments | `3 bits`                 | The number of fragments in a packet.                              | *Varies*                      |
| Is file?            | `1 bit`                  | If the message contains a file                                    | *Its binary*                  |
| Transfer ID         | `u32`                    | The transfer ID of the file. **Only read if is a file**           | *Varies*                      |
| Filename            | `string`                 | The filename of the file. **Only read if is a file**              | *Varies*                      |
| Is compressed?      | `1 bit`                  | If the message is compressed.                                     | *Its binary*                  |
| Uncompressed size   | `26 bits`                | The uncompressed size of the data. **Only read if is compressed** | *Varies*                      |
| Bytes               | `26 bits`                | The size of the entire file in bytes.                             | *Varies*                      |
| Data                | `bytes+(1 << 8)-1 bytes` | The data being sent.                                              | *Varies*                      |

###### Following Blocks

| Name                | Type                             | Description                          | Value                         |
|---------------------|----------------------------------|--------------------------------------|-------------------------------|
| Single Block?       | `bit`                            | If the message is a single block     | `1` means **multiple blocks** |
| Start Fragment      | `26 - 8 bits`                    | The start fragment of the packet     | *Varies*                      |
| Number of fragments | `3 bits`                         | The number of fragments in a packet. | *Varies*                      |
| Data                | `Number of fragments * (1 << 8)` | The data being sent                  | *Varies*                      |

## NetChannel Commands (Follow SubChannel Data)

### Basic structure

| Name    | Type     | Description                              | Value    |
|---------|----------|------------------------------------------|----------|
| Command | `6 bits` | The command being issued                 | *Varies* |
| Data    | *Varies* | The command body, could be none or some. | *Varies* |

#### NetChannel Commands

##### Control Commands

###### net_NOP = `0x00`

This message has no body, it is padding

###### net_Disconnect = `0x01`

| Name   | Type     | Description               | Value    |
|--------|----------|---------------------------|----------|
| Reason | `string` | The reason for disconnect | *Varies* |

###### net_File = `0x02`

| Name        | Type     | Description                     | Value        |
|-------------|----------|---------------------------------|--------------|
| Transfer ID | `u32`    | The transfer ID of the file     | *Varies*     |
| File name?  | `string` | The name of the file requested? | *Varies*     |
| Deny?       | `bit`    | Denies the transfer if not 1    | *Its binary* |
