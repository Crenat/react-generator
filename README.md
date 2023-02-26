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

Usage: `rg <type> <name>`

---

### The list of types
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

  In future there will be flags and `rc` file for preferences and your personalised templates

  `Crenat` <martsenyuk.yura@gmail.com>