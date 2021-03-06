# onboarding-rust

### First review if you have set up your Joy Labs [development setup](https://honey.is/home/#post/778734)

1.- [Create an SSH key on your local computer.](https://help.github.com/en/enterprise/2.15/user/articles/adding-a-new-ssh-key-to-your-github-account)

2.- [Check if you have git installed, if not install it:](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
```
git --version
```

3.- On the folder you select, clone the master repository with SSH
```
git clone SSH_link
```

4.- Check repository branches and in which one you are:
```
git branch
```

5.- Create and move to a new branch with your name (this will be your master personal repository):
```
git checkout -b your_name
```

6.- Create and move to a new branch for your exercise:
```
git checkout -b your_name-e1
```

7.- Solve your exercise, follow the structure of the folders and 
*To run the test navigate to that days submission and run*

```
cargo test
```

8.- Once the tests for your exercise pass, run the comprehensive tests for the entire project
*To run the test navigate to the root of onboarding-rust repo and run*

```
docker-compose run build bin/test
```

9.- Add all your changes (you can review your modified files with *git status*):
```
git add .
```

10.-Create your commit:
```
git commit -m "your comments here"

```

11.- Push changes on your branch:
```
git push origin name_of_the_branch
```

12.- On Github make a pull request of your branch and choose a peer to review it (try to choose someone different).  Check that the automated checks on the PR all pass, and fix any that don't.


### Resources

  - [Git tutorial](https://learngitbranching.js.org)