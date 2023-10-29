# self?

module_function
ref: https://github.com/ruby/gem_rbs_collection/compare/main...m11o:feature/add-types-for-security-utils?expand=1#diff-6f8f3883c6886400ad277b462919f4f8209b4902fe367ca12899cf98e3d93ed1R157

# ? の使い方

? String ?
前方の `?` はoptionalという意味
後方の `?` はnilの可能性があるということ

# interned

String | Symbol みたいな型は `interned` を使用する
Object::nameを使用している箇所がgem_rbs_collection内にありそうなので、リリース後に修正する