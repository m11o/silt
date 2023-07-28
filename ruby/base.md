# privateとprotectedの違い

doc: https://docs.ruby-lang.org/ja/latest/doc/spec=2fdef.html#limit

- public: 制限なし
- private: クラスの外から呼び出すことができない。同じインスタンス内でのみ、関数形式で呼び出せる。
- protected: クラスの外から呼び出すことができない。同じインスタンス内では呼び出すことが可能。また、他のインスタンスであっても同じクラスやサブクラスであれば呼び出せる

Matz曰く

```text
つまり，privateは自分からしか見えないメソッドであるのに対し
て，protectedは一般の人からは見られたくないが，仲間(クラスが
同じオブジェクト)からは見えるメソッドです．

protectedは例えば2項演算子の実装にもう一方のオブジェクトの状
態を知る必要があるか調べる必要があるが，そのメソッドをpublic
にして，広く公開するのは避けたいというような時に使います．
```

```ruby
class PrivateProtectedClass
  def public_method
    puts "public"
  end
  
  def call_protected_method
    protected_method
  end
  
  def call_protected_method_with_self
    self.protected_method
  end
  
  def call_other_protected_method(other)
    other.protected_method
  end
  
  def call_private_method
    private_method
  end
  
  def call_private_method_with_self
    self.private_method
  rescue
    puts 'failed call private method with self'
  end
  
  def call_other_private_method(other)
    other.private_method
  rescue
    puts 'failed call other private method'
  end 
  
  protected
  
  def protected_method
    puts "protected"
  end
  
  private
  
  def private_method
    puts "private"
  end
end

p1 = PrivateProtectedClass.new
p2 = PrivateProtectedClass.new

p1.public_method
begin
  p1.protected_method
rescue
  puts 'failed call protected method directly'
end

begin
  p1.private_method
rescue
  puts 'failed call private method directly'
end

p1.call_protected_method
p1.call_protected_method_with_self
p1.call_other_protected_method(p2)

p1.call_private_method
p1.call_private_method_with_self
p1.call_other_private_method(p2)
```

references
https://qiita.com/tbpgr/items/6f1c0c7b77218f74c63e

