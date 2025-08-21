# Contributing to Clutch Node

## 🔒 **Branch Protection & PR Requirements**

Clutch Node uses **branch protection rules** to maintain code quality while enabling rapid development. Contributors must use Pull Requests, but maintainers can bypass restrictions.

## 📋 **Branch Protection Rules**

### **Main Branch (`main`)**
- ✅ **Requires PR** before merging
- ✅ **1 approving review** required
- ✅ **Status checks** must pass (build, test, lint)
- ✅ **Branches must be up to date** before merging
- ❌ **No force pushes** allowed
- ❌ **No deletions** allowed

### **Develop Branch (`develop`)**
- ✅ **Requires PR** before merging
- ✅ **1 approving review** required
- ✅ **Status checks** must pass (build, test)
- ❌ **No force pushes** allowed
- ❌ **No deletions** allowed

### **Release Branches (`release/*`)**
- ✅ **Requires PR** before merging
- ✅ **2 approving reviews** required
- ✅ **Code owner review** required
- ✅ **All status checks** must pass
- ❌ **No force pushes** allowed
- ❌ **No deletions** allowed

## 🔓 **Maintainer Bypass (Mehran Mazhar)**

As the project maintainer, **Mehran Mazhar** can:
- ✅ **Push directly** to protected branches
- ✅ **Bypass PR requirements** for rapid development
- ✅ **Dismiss stale reviews**
- ✅ **Force push** when necessary (use responsibly)

## 🚀 **Contributor Workflow**

### **1. Create Feature Branch**
```bash
git checkout -b feature/your-feature-name
```

### **2. Make Changes & Commit**
```bash
# Make your changes
git add .
git commit -m "Feature: Your feature description"
```

### **3. Push & Create PR**
```bash
git push origin feature/your-feature-name
# Create Pull Request on GitHub
```

### **4. Wait for Reviews & Checks**
- **Status checks** must pass (build, test, lint)
- **At least 1 approval** required
- **Branch must be up to date** with main

### **5. Merge When Ready**
Once approved and checks pass, your PR can be merged!

## 🔧 **Required Status Checks**

| Check | Description | Required For |
|-------|-------------|--------------|
| **build** | Rust compilation | All branches |
| **test** | Unit & integration tests | All branches |
| **lint** | Code formatting & linting | Main & release |
| **security-scan** | Security vulnerability scan | Release branches |

## ⚡ **Maintainer Quick Development**

For rapid MVP development, Mehran Mazhar can:

```bash
# Direct push to main (bypasses protection)
git push origin main

# Direct push to develop (bypasses protection)
git push origin develop

# Force push when necessary (use responsibly)
git push origin main --force
```

## 🎯 **Why This Setup?**

For Clutch Protocol's **12-week MVP timeline**:
- **Code quality** through PR reviews
- **Rapid iteration** for maintainers
- **Community contribution** with proper workflow
- **Security** through status checks

## ⚠️ **Important Notes**

- **Contributors cannot force push** to protected branches
- **All PRs require approval** (except maintainer's)
- **Status checks must pass** before merging
- **Use maintainer bypass responsibly**

## 🚀 **Getting Started**

1. **Fork** the clutch-node repository
2. **Clone** your fork locally
3. **Create feature branch** for your changes
4. **Submit PR** and wait for review
5. **Join** the decentralized blockchain future!

## 📞 **Need Help?**

- Create an issue for questions
- Join GitHub Discussions
- Check the main README for project details

---

**Remember**: This setup balances code quality with development speed. Contributors use PRs, maintainers can bypass for rapid iteration!