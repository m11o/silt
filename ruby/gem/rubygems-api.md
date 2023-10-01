# RubyGems API

APIがあるみたい
https://guides.rubygems.org/rubygems-org-api-v2/
これはV2っぽい

```shell
curl https://rubygems.org/api/v2/rubygems/rails/versions/7.0.8.json
```

```json
{
  "name": "rails",
  "downloads": 467544645,
  "version": "7.0.8",
  "version_created_at": "2023-09-09T19:15:48.031Z",
  "version_downloads": 720675,
  "platform": "ruby",
  "authors": "David Heinemeier Hansson",
  "info": "Ruby on Rails is a full-stack web framework optimized for programmer happiness and sustainable productivity. It encourages beautiful code by favoring convention over configuration.",
  "licenses": [
    "MIT"
  ],
  "metadata": {
    "changelog_uri": "https://github.com/rails/rails/releases/tag/v7.0.8",
    "bug_tracker_uri": "https://github.com/rails/rails/issues",
    "source_code_uri": "https://github.com/rails/rails/tree/v7.0.8",
    "mailing_list_uri": "https://discuss.rubyonrails.org/c/rubyonrails-talk",
    "documentation_uri": "https://api.rubyonrails.org/v7.0.8/",
    "rubygems_mfa_required": "true"
  },
  "yanked": false,
  "sha": "8e43af921acf766fb429126f020ec90c3b25809631f8fbdff95c3553828d5867",
  "project_uri": "https://rubygems.org/gems/rails",
  "gem_uri": "https://rubygems.org/gems/rails-7.0.8.gem",
  "homepage_uri": "https://rubyonrails.org",
  "wiki_uri": null,
  "documentation_uri": "https://api.rubyonrails.org/v7.0.8/",
  "mailing_list_uri": "https://discuss.rubyonrails.org/c/rubyonrails-talk",
  "source_code_uri": "https://github.com/rails/rails/tree/v7.0.8",
  "bug_tracker_uri": "https://github.com/rails/rails/issues",
  "changelog_uri": "https://github.com/rails/rails/releases/tag/v7.0.8",
  "funding_uri": null,
  "dependencies": {
    "development": [],
    "runtime": [
      {
        "name": "actioncable",
        "requirements": "= 7.0.8"
      },
      {
        "name": "actionmailbox",
        "requirements": "= 7.0.8"
      },
      {
        "name": "actionmailer",
        "requirements": "= 7.0.8"
      },
      {
        "name": "actionpack",
        "requirements": "= 7.0.8"
      },
      {
        "name": "actiontext",
        "requirements": "= 7.0.8"
      },
      {
        "name": "actionview",
        "requirements": "= 7.0.8"
      },
      {
        "name": "activejob",
        "requirements": "= 7.0.8"
      },
      {
        "name": "activemodel",
        "requirements": "= 7.0.8"
      },
      {
        "name": "activerecord",
        "requirements": "= 7.0.8"
      },
      {
        "name": "activestorage",
        "requirements": "= 7.0.8"
      },
      {
        "name": "activesupport",
        "requirements": "= 7.0.8"
      },
      {
        "name": "bundler",
        "requirements": "\u003e= 1.15.0"
      },
      {
        "name": "railties",
        "requirements": "= 7.0.8"
      }
    ]
  },
  "built_at": "2023-09-09T00:00:00.000Z",
  "created_at": "2023-09-09T19:15:48.031Z",
  "description": "Ruby on Rails is a full-stack web framework optimized for programmer happiness and sustainable productivity. It encourages beautiful code by favoring convention over configuration.",
  "downloads_count": 720675,
  "number": "7.0.8",
  "summary": "Full-stack web application framework.",
  "rubygems_version": "\u003e= 1.8.11",
  "ruby_version": "\u003e= 2.7.0",
  "prerelease": false,
  "requirements": [],
  "spec_sha": null
}
```

V1なら色々できそう
でもラベルみたいな情報はそもそも取れなさそう

ref: https://guides.rubygems.org/rubygems-org-api/