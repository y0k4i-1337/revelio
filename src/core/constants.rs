use phf::phf_map;

// To see the full list of scopes, see
// https://docs.microsoft.com/en-us/graph/permissions-reference
// The following scopes does not require admin consent
pub const DEFAULT_SCOPES: &str = "openid,profile,email,User.Read,User.ReadBasic.All";

// pub const DEFAULT_CLIENT_ID: &str = "de8bc8b5-d9f9-48b1-a8ad-b748da725064";
pub const DEFAULT_CLIENT_ID: &str = "d924a533-3729-4708-b3e8-1d2445af35e3"; // skype

pub const USER_AGENTS_KEYS: [&str; 10] = [
    "android",
    "apple_iphone_safari",
    "apple_mac_firefox",
    "linux_firefox",
    "win_chrome_win10",
    "win_ie11_win7",
    "win_ie11_win8",
    "win_ie11_win8.1",
    "win_ie11_win10",
    "win_edge_win10",
];

pub const USER_AGENTS: phf::Map<&'static str, &'static str> = phf_map! {
    "android" => "Mozilla/5.0 (Linux; Android 12) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.50 Mobile Safari/537.36",
    "apple_iphone_safari" => "Mozilla/5.0 (iPhone; CPU iPhone OS 15_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1",
    "apple_mac_firefox" => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:94.0) Gecko/20100101 Firefox/94.0",
    "linux_firefox" => "Mozilla/5.0 (X11; Linux i686; rv:94.0) Gecko/20100101 Firefox/94.0",
    "win_chrome_win10" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36",
    "win_ie11_win7" => "Mozilla/5.0 (Windows NT 6.1; Trident/7.0; rv:11.0) like Gecko",
    "win_ie11_win8" => "Mozilla/5.0 (Windows NT 6.2; Trident/7.0; rv:11.0) like Gecko",
    "win_ie11_win8.1" => "Mozilla/5.0 (Windows NT 6.3; Trident/7.0; rv:11.0) like Gecko",
    "win_ie11_win10" => "Mozilla/5.0 (Windows NT 10.0; Trident/7.0; rv:11.0) like Gecko",
    "win_edge_win10" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.69 Safari/537.36 Edg/95.0.1020.44",
};
