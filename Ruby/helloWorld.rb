puts 'Hello World'
# def method
#   puts "Hello World from Method"
# end
#
# def hello
#   puts "Hello #{name}! Have a great day!"
#
# end
# hello ("Alex")

class Greeter
  def initialize(name = 'AI')
    @name = name
    end

  def hello
    puts "Hey #{@name}!"
  end

  def goodbye
    puts "Bye #{@name}, have a good day! Come back and talk soon."
  end
end
greeter=Greeter.new("Alex")
greeter.hello

greeter.goodbye
