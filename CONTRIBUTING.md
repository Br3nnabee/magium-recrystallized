# Contributing to Magium Recrystallized

Thank you for considering contributing to *Magium Recrystallized*! We welcome all forms of contributions - code, design, writing, bug reports, feature suggestions, and more.

## 📋 Code of Conduct
By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Be respectful and considerate in your interactions.

## 🛠️ How to Contribute
### 1. Set Up the Project
Make sure you can run the game locally. Instructions can be found in the [README](https://github.com/Br3nnabee/magium-recrystallized/blob/main/README.md)

### 2. Pick an Issue or Suggest One
Check the [Issues](https://github.com/br3nnabee/magium-recrystallized/issues) tab. Look for labels such as `good first issue` or `help wanted`. You may also open a new issue to propose improvements or report bugs.

### 3. Branch and Code
Create a descriptive branch name:
```bash
git checkout -b feature/AmazingFeature
```
Follow our conventions and ensure your code is clean and well-documented.

### 4. Test Your Changes
Please test your changes thoroughly. Make sure you haven’t broken existing features or introduced bugs. Although we don't require integration or unit tests, you may use them on your end and leave them in during your commit.

### 5. Commit and Push
Use meaningful commit messages:
```bash
git commit -m "feat: add new amazing feature using this and that"
git push origin feature/AmazingFeature
```

### 6. Open a Pull Request
Submit a pull request from your branch to the `dev` branch. Fill out the PR template to help us understand your changes.

## 🧪 Coding Standards
- Prefer descriptive variable and function names.
- Keep logic modular and components isolated.
- Follow formatting rules enforced by the linter.

## 📄 Project Structure Overview
```
┌── src-tauri/             # Code for the tauri builds. Largely untouched.
├── src/                   # Code for the web app.
│   ├── lib/               # Utility functions etc.
|   |   ├── components/    # Components that are loaded with svelte.
|   |   ├── stores/        # Typescript functions defining things stored.
|   └── routes/            # The different pages. Largely untouched.
├── static/                # All static content (images etc.).
└── wasm_module/           # Code for heavier logic such as decoding.
```

## ❤️ A Note of Thanks
This project exists thanks to contributors like you. Whether you’re here to fix a typo or design a whole new module, your input is valuable.

Thank you for helping bring *Magium Recrystallized* to life.

— Br3nnabee and the Magium Community
