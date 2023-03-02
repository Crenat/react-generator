# react-generator

#### A CLI for generating file and directory structure like in Angular `ng`

___

## Instalation
1. Install Rust
2. ```bash
    cargo build --release

    target/release/react-generator
    ```
3. add alias to your shell in rc file

    ```bash
    alias rg="PATH-TO-BINARY"
    ```
## Usage
`rg` - the root command for react-generator

Usage: `rg <operation> <name>`

Also you can create several elements at the same time in this way `rg <operation> <name> <name> <name> ...`

---

### The list of operations
- `module` - generates the `modules` folder if it doesn't exist yet, in it generates an the following folders `api`, `components`, `hooks`, `layouts`, `pages`. Also the module has an `index.ts` file in witch there is the followind stub:
  ```typescript
  import * as api from './api';
  import * as pages from './pages';

  const mainModule = {
    pages: { ...pages },
    api: { ...api },
  }

  export default mainModule;
  ```
- `component` - generates the `components` folder if it doesn't exist yet, and generates folder with the component's name, in it generates an `index.ts` file with the following stub:
  ```typescript
  import React from 'react';

  interface ButtonProps {
      // Add props here
  }

  const Button = ({}: ButtonProps) => {
      return <div>Button</div>;
  };

  export default Button;
  ```
- `layout` - generates the `layouts` folder if it doesn't exist yet, and generates folder with the layout's name, in it generates an `index.ts` file with the following stub:
  ```typescript
  import React from 'react';

  interface UsersProps {
      // Add props here
  }

  const Users = ({ ...props }: UsersProps) => {
      return (
          <div>
              Users Layout
          </div>
      );
  };

  export default Users;
  ```

  ## Config
  You can create a `.rgrc.toml` file in the root of your project. You can specify the root folder for creating the folders that gonna be created by `react-generator`. For default React application this is a `src` folder, this is also the default value

  ### Here is an example of `.rgrc.toml`
  ```toml
  root_folder = "."
  ```

  In future there will be flags and your personalised templates

  [!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/gbraad)

  `Crenat` <martsenyuk.yura@gmail.com>