<div align="center" id="top">
  <img src="./.github/revelio.jpg" width=50% alt="Revelio" />

  &#xa0;

  <!-- <a href="https://revelio.netlify.app">Demo</a> -->
</div>

<h1 align="center">Revelio</h1>

<p align="center">
  <img alt="Github top language" src="https://img.shields.io/github/languages/top/y0k4i-1337/revelio?color=56BEB8">

  <img alt="Github language count" src="https://img.shields.io/github/languages/count/y0k4i-1337/revelio?color=56BEB8">

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/y0k4i-1337/revelio?color=56BEB8">

  <img alt="License" src="https://img.shields.io/github/license/y0k4i-1337/revelio?color=56BEB8">

  <!-- <img alt="Github issues" src="https://img.shields.io/github/issues/y0k4i-1337/revelio?color=56BEB8" /> -->

  <!-- <img alt="Github forks" src="https://img.shields.io/github/forks/y0k4i-1337/revelio?color=56BEB8" /> -->

  <!-- <img alt="Github stars" src="https://img.shields.io/github/stars/y0k4i-1337/revelio?color=56BEB8" /> -->
</p>


<p align="center">
  <a href="#dart-about">About</a> &#xa0; | &#xa0;
  <a href="#sparkles-features">Features</a> &#xa0; | &#xa0;
  <a href="#rocket-technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#white_check_mark-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#checkered_flag-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#crystal_ball-usage">Usage</a> &#xa0; | &#xa0;
  <a href="#memo-license">License</a> &#xa0; | &#xa0;
  <a href="https://github.com/y0k4i-1337" target="_blank">Author</a>
</p>

<br>

## :dart: About ##

Revelio is a versatile Rust command-line tool that simplifies interaction with the Microsoft Graph API for authentication and data retrieval.

## :sparkles: Features ##

:heavy_check_mark: Secure authentication with the Microsoft Graph API using OAuth2;\
:heavy_check_mark: Retrieve basic users information from your Azure Active Directory tenant.

## :rocket: Technologies ##

The following tools were used in this project:

- [Rust](https://rust-lang.org)
- [Microsoft Graph API](https://learn.microsoft.com/en-us/graph/overview)
- [oauth2](https://docs.rs/oauth2/latest/oauth2/)
- [tokio](https://docs.rs/tokio/latest/tokio/)

## :white_check_mark: Requirements ##

Before starting :checkered_flag:, you need to have [Git](https://git-scm.com)
and [Rust](https://rust-lang.org) installed.

### Linux

If you encounter problem while building this project related to `pkg-config`,
you may have to install it:
  - On Debian/Ubuntu: `sudo apt-get install pkg-config`
  - On Fedora: `sudo dnf install pkgconfig`
  - On CentOS/RHEL: `sudo yum install pkgconfig`

## :checkered_flag: Starting ##

```bash
# Clone this project
$ git clone https://github.com/y0k4i-1337/revelio.git

# Access
$ cd revelio

# Build the project
$ cargo build --release

# Run the project
$ ./target/release/revelio -h
```

## :crystal_ball: Usage ##

```
./target/release/revelio -h
Reveals data from Microsoft Tenants using the Microsoft Graph API

Usage: revelio [OPTIONS] <COMMAND>

Commands:
  get   Get resources in a tenant
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --client-id <CLIENT_ID>          Custom client ID to use for API requests
  -s, --client-secret <CLIENT_SECRET>  Custom client secret to use for API requests
  -t, --tenant-id <TENANT_ID>          Tenant ID to use for API requests [default: common]
  -S, --scopes <SCOPES>                Comma-separated list of scopes to use for API requests [default:
                                       openid,profile,email,User.Read,User.ReadBasic.All]
  -k, --access-token <ACCESS_TOKEN>    Set access token to use for API requests [env: REVELIO_TOKEN=]
  -v, --api-version <API_VERSION>      API version to use for API requests [default: v1] [possible values: v1, beta]
  -U, --user-agent <USER_AGENT>        User-agent to use for API requests [default: win_chrome_win10] [possible values:
                                       android, apple_iphone_safari, apple_mac_firefox, linux_firefox, win_chrome_win10,
                                       win_ie11_win7, win_ie11_win8, win_ie11_win8.1, win_ie11_win10, win_edge_win10]
  -x, --proxy <PROXY>                  Set proxy to use for API requests (except for authentication)
  -i, --ignore-ssl                     Ignore SSL certificate verification
  -o, --out-dir <OUT_DIR>              Output directory (only used when retrieving large amounts of data) [default: .]
  -h, --help                           Print help
  -V, --version                        Print version
```

Menu for `get` subcommand:

```
./target/release/revelio get -h
Get resources in a tenant

Usage: revelio get [OPTIONS] <RESOURCE>

Arguments:
  <RESOURCE>  Resource to get [possible values: me, users, users-count]

Options:
      --select <SELECT>        Custom select query parameter (properties to return)
      --top <TOP>              Custom top query parameter (page size of results) [default: 500]
      --skiptoken <SKIPTOKEN>  Set skiptoken to continue from a previous request
      --pages <PAGES>          Maximum number of pages to return (0 for all pages) [default: 0]
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```

## :memo: License ##

This project is under license from MIT. For more details, see the [LICENSE](LICENSE.md) file.


Made with :heart: by <a href="https://github.com/y0k4i-1337" target="_blank">y0k4i</a>

&#xa0;

<a href="#top">Back to top</a>
