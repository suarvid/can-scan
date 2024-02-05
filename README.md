# CAN-Scan: Verify expected CAN messages via the Command Line
Tool for programatically verifiying that expected CAN frames are received.
Specify which CAN frames are expected, in the form of an id, dlc, and data field contents, as well as an optional name.
Then, run the program with a given can interface, the specified YAML file, and the number of frames to sample.

In situations where the same set of CAN frames are typically always expected, and this needs to be verified between changes of the code sending the messages,
this tool aims to make it simpler to verify that the CAN messages are sent as expected, compared to manually verifying that the messages are sent using
a tool such as SavyCAN or Busmaster.

## Example Usage
`can-scan can0 expected_messages.yml 1000`
Or, if cloning this repository:
`cargo run can0 expected_messages.yml 1000`

This will sample 1000 CAN frames on the `can0` interface, and verify that all messages in `expected_messages.yml` were received as specified.
If any specified message was not recieved as expected, this will be printed to the terminal.