<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">Woopra component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/woopra-component/badge.svg)](https://coveralls.io/github/edgee-cloud/woopra-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/woopra-component.svg)](https://github.com/edgee-cloud/woopra-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/woopra)


This component enables seamless integration between [Edgee](https://www.edgee.cloud) and [Woopra](https://www.woopra.com/), allowing you to collect and forward analytics events to your Woopra project.


## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `woopra.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[destinations.data_collection]]
id = "woopra"
file = "/var/edgee/components/woopra.wasm"
settings.project = "example.com"
```


## Event Handling

### Event Mapping
The component maps Edgee events to Woopra events as follows.

| Edgee Event | Woopra event                   | Description                          |
|-------------|--------------------------------|--------------------------------------|
| Page        | Track request with name = "pv" | Triggered when a user views a page   |
| Track       | Track request                  | Triggered for custom events          |
| User        | Identify request               | Use it to update visitor properties  |


## Configuration Options

### Basic Configuration
```toml
[[destinations.data_collection]]
id = "woopra"
file = "/var/edgee/components/woopra.wasm"
settings.project = "example.com"
```

### Event Controls
Control which events are forwarded to Woopra:
```toml
settings.edgee_page_event_enabled = true   # Enable/disable page view tracking
settings.edgee_track_event_enabled = true  # Enable/disable custom event tracking
settings.edgee_user_event_enabled = true   # Enable/disable user identification
```


## Development

### Building from Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)

Build command:
```bash
edgee component build
```

Test command:
```bash
make test
```

Test coverage command:
```bash
make test.coverage[.html]
```

### Contributing
Interested in contributing? Read our [contribution guidelines](./CONTRIBUTING.md)

### Security
Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud)
