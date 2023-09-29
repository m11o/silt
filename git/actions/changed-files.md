# 変更されたファイル一覧を取得する方法

github actions内で変更したファイルのみ取得するlibraryがあったのでメモ
ref: https://github.com/tj-actions/changed-files

actions内でこのlibraryを読み込むと変更ファイル一覧を変数に入れてくれるっぽい

sample

```
      - name: Get changed files
        id: changed-files
        uses: tj-actions/changed-files@v39
        
        # To compare changes between the current commit and the last pushed remote commit set `since_last_remote_commit: true`. e.g
        # with:
        #   since_last_remote_commit: true 

      - name: List all changed files
        run: |
          for file in ${{ steps.changed-files.outputs.all_changed_files }}; do
            echo "$file was changed"
          done
```