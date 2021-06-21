#include <bits/stdc++.h>

namespace core
{
    struct RAM
    {
        std::unordered_map<size_t, size_t> memory;
    };

    size_t read_volatile(size_t address, RAM ram)
    {
        return ram.memory[address];
    }

    void write_volatile(size_t value, size_t address, RAM ram)
    {
        try
        {
            ram.memory[address] = value;
            std::cout << value << " written at address " << address << std::endl;
        }
        catch (const std::exception &e)
        {
            std::cerr << e.what() << '\n';
        }
    }

    void _nop()
    {
        std::cout << "Stopping for 1 clock cycle!";
    }

    template <typename T>
    T newObject(size_t address)
    {
        T ptr(address);
        return ptr;
    }
} // namespace core
