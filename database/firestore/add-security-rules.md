デフォルトでは、すべてのリクエストを禁止する設定になっている

```
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    match /{document=**} {
      allow read, write: if false;
    }
  }
}
```

firebase cliなどでSecurity Ruleを変更し、デブロイすることでSecurity Ruleを変更することができる

全リクエストを許可

```
// Allow read/write access to all users under any conditions
// Warning: **NEVER** use this rule set in production; it allows
// anyone to overwrite your entire database.
service cloud.firestore {
  match /databases/{database}/documents {
    match /{document=**} {
      allow read, write: if true;
    }
  }
}
```

認証必須

```
// Allow read/write access on all documents to any user signed in to the application
service cloud.firestore {
  match /databases/{database}/documents {
    match /{document=**} {
      allow read, write: if request.auth != null;
    }
  }
}
```

testモードで、作成することができこの場合には1ヶ月間read, write権限が付与される模様
productionモードは外部の3rd party製clientではアクセスできないみたい