# Create Cursor App


<p align="center">
  Create Cursor IDE projects with no configuration.
</p>

<p align="center">
  <a href="#quick-overview">Quick Overview</a> •
  <a href="#creating-an-app">Creating an App</a> •
  <a href="#templates">Templates</a> •
</p>

Create Cursor App works on macOS, Windows, and Linux.<br>
If something doesn't work, please [file an issue](https://github.com/ozankasikci/create-cursor-app/issues/new).

## Quick Overview

```sh
npx create-cursor-app my-project
cd my-project
```

Your new Cursor IDE project will be created with all the necessary configuration and documentation files.

## Creating an App

**You'll need to have Node.js 14.0.0 or later version on your local development machine**. We recommend using the latest LTS version.

### npx

```sh
npx create-cursor-app my-project
```

### npm

```sh
npm init cursor-app my-project
```

### Yarn

```sh
yarn create cursor-app my-project
```

It will create a directory called `my-project` inside the current folder with the following structure:

```
my-project
├── PROMPT.md
├── CHANGELOG.md
├── PROJECT_SCOPE.md
├── .cursorrules
└── [template specific files]
```

## Templates

The following templates are available:

- `basic` - A basic template with essential Cursor IDE configuration
- More templates coming soon...

## License

Create Cursor App is open source software [licensed as MIT](https://github.com/ozankasikci/create-cursor-app/blob/main/LICENSE). 