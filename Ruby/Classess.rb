class Car
  attr_accessor :color
  attr_reader :make, :model
  def initialize (make, model, color)
        @make = make
        @model = model
        @color = color
    end

    def turn(direction)
    end

    def honk
        puts "beep beep"
    end

    def brake
    end
end

civic = Car.new('Honda', 'Civic Hatchback', 'Blue')
