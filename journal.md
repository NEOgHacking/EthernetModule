
# Journal 1 20m

# Brainstorm
So long story short i wanted internet and specifically Ethernet because who doesn't love a wire :yay: Well i did a bit of digging and most of the phys i found used in most devices need a special part in the microcontroller so to prevent that i went for the good old W5500 SPI ethernet phy. Then i went for the best selling Ethernet connector with integrated magnetics.

![image](https://cdn.hackclub.com/01a0e98b-dcf2-70fe-a25e-6186ae13a93e/image.png)

Apart from that it will be a very tight module but o well challenges are fun.

# Journal 2 90m

# Schematic
So at first i laid out all the ic's and connectors i needed. With the things i choose in the last journal

![image](https://cdn.hackclub.com/01a0e98b-b2c4-7bd4-8ef8-b98053336102/screenshot_from_2026-09-28_16-52-09.png)

Then i started following the datasheet of the W5500 for the right capacitances and resistances needed for the different pins used first i started with the pins that were defined what stuff they needed in the pinout diagram.

![image](https://cdn.hackclub.com/01a0e98b-aeec-7997-9b11-c9502e4d2ecf/screenshot_from_2026-09-28_17-00-11.png)

Then i found the specs of the crystal i needed. I directly implemented that into the schematic as per datasheet rules.

![image](https://cdn.hackclub.com/01a0e98b-ab3d-7228-9c69-919149369034/screenshot_from_2026-09-28_17-10-11.png)

After this i dug a bit deeper and did allot of decoupling for the voltage in.

![image](https://cdn.hackclub.com/01a0e98b-b678-7f3f-bf7a-91136419d2e2/screenshot_from_2026-09-28_17-19-33.png)

Then i started on the Ethernet connector, at first i did the status leds for the ethernet connector so it would show the rights stuff you would expect it to show.

![image](https://cdn.hackclub.com/01a0e98b-a790-7e14-804c-130a2bf7f165/screenshot_from_2026-09-28_17-10-17.png)

Then i started on the hardest part, the decoupling and resistors needed for the tx and rx pairs of the Ethernet connection, this is after 2 times still black magic for me but i just follow the datasheet of the W5500.

![image](https://cdn.hackclub.com/01a0e98b-a40c-7a0b-8053-a7ae50c90893/screenshot_from_2026-09-28_17-10-21.png)

After some tries i got it done and had it laid out nicely that i was happy with.

![image](https://cdn.hackclub.com/01a0e98b-a034-79d6-b89b-c18a11b6cb21/screenshot_from_2026-09-28_17-19-37.png)

Then i did the right labels for the Ethernet ic, the spi pins and the reset pins ext. the rest i stopped of with those cross thingies.

![image](https://cdn.hackclub.com/01a0e98b-9c46-7e12-8887-56929fde293e/screenshot_from_2026-09-28_17-19-42.png)

Now it was time to connect it to the module connector. This was a bit of a unknown. I only connected the pins that would need to go to specific pins so i could choose the pinout for the others while doing the PCB design so it would match the PCB better.

![image](https://cdn.hackclub.com/01a0e98b-98b7-7cd4-b306-ac4f28077394/screenshot_from_2026-09-28_17-22-18.png)

Then at the end a total of the schematic.

![image](https://cdn.hackclub.com/01a0e98b-952b-7f5e-b548-89b5defed172/screenshot_from_2026-09-28_17-29-17.png)


# Journal 3 300m

# PCB layout and routing

At first i started with the layout of the bigest components like the network connector the ic and the expansion connector, these would be the most irritatingly in the way so i wanted to get them out of the way as soon as possible.

![image](https://cdn.hackclub.com/01a0e99d-23d7-708f-b7c4-1c8854e51b6d/screenshot_from_2026-09-28_17-32-46.png)

Then i started with routing the most important traces off them all the ethernet pairs. I used the calculator from jlcpcb to calculate the impedance needed, tho for this short length it wouldn't matter much.

![image](https://cdn.hackclub.com/01a0e99d-2000-7fc3-95cc-78d0bacdc050/screenshot_from_2026-09-28_17-35-30.png)

now it was time to do the next thing This was where i would want all the components, i also looked at other ethernet connectors but 90% of them where to long so this was the way to go.

![image](https://cdn.hackclub.com/01a0e99d-1c32-7356-ac71-bca722e9dba7/screenshot_from_2026-09-28_17-42-17.png)

Then i started with decoupling the ic and also adding the crystal for the ic, i decided on a 2 sided pcb because of the space constraints.

![image](https://cdn.hackclub.com/01a0e99d-1774-7a54-b119-904b1fe26654/screenshot_from_2026-09-28_18-12-39.png)

Now i had some more components placed down i needed to length match the tx trace, i did this and this worked, but just barely.

![image](https://cdn.hackclub.com/01a0e99d-13d8-74fe-93ff-4b30cc20b5ce/screenshot_from_2026-09-28_18-23-09.png)

Next things next I started thinking about how i wanted to lay out the rx trace, it was really aquard spot and i had tried a few different things. Here i had a general idea of where i wanted the components that where needed in between the rx and tx traces.

![image](https://cdn.hackclub.com/01a0e99d-0f9b-78e0-b2ff-8665af45792e/screenshot_from_2026-09-28_18-24-25.png)

The next picture was a small step of brainstorming the ethernet traces.

![image](https://cdn.hackclub.com/01a0e99d-0bcc-70fe-9e68-5fb6ead25953/screenshot_from_2026-09-28_18-27-59.png)

Then i routed the RX side and found out this is what i had to deal with.

![image](https://cdn.hackclub.com/01a0e99c-62b0-7ba1-81d1-2e362743e9d4/screenshot_from_2026-09-28_18-30-06.png)

So i tried a few different things but ended up here with this botched up trace so + and - of the RX trace would line up.

![image](https://cdn.hackclub.com/01a0e99c-5e0a-742a-a065-4373ccb82b8d/screenshot_from_2026-09-28_18-37-38.png)

Then it was time for some length matching, and some was an understatement, 5mm for this was crazy but eah its what we have to do, im just gradefull i didn't need to lenght match rx to tx that would be terror.

![image](https://cdn.hackclub.com/01a0e99c-59e8-777e-a68b-14e272b215e9/screenshot_from_2026-09-28_19-57-47.png)

Now time for the length matching here is its beutifull sight.

![image](https://cdn.hackclub.com/01a0e99c-55e1-7990-a887-fc1bdc30859a/screenshot_from_2026-09-28_20-00-30.png)

Then i routed the 3.3V power trace and the spi bus.

![image](https://cdn.hackclub.com/01a0e99c-51a8-7303-bbb5-880f24a9f022/screenshot_from_2026-09-28_20-11-01.png)

Now at the end, here are a few final pictures of the end product.

![image](https://cdn.hackclub.com/01a0e99c-4d5c-7819-a062-99889655f200/screenshot_from_2026-09-28_20-12-07.png)



![image](https://cdn.hackclub.com/01a0e99c-480e-72aa-b6f4-c4d357f7eced/screenshot_from_2026-09-28_20-12-09.png)



![image](https://cdn.hackclub.com/01a0e99c-442a-71c8-ab0f-59d51a2b6c56/screenshot_from_2026-09-28_20-16-30.png)



![image](https://cdn.hackclub.com/01a0e99c-400c-7a00-b128-625910b097d1/screenshot_from_2026-09-28_20-16-38.png)



![image](https://cdn.hackclub.com/01a0e99c-3afe-76c7-a9b6-92463f403211/screenshot_from_2026-09-28_20-44-51.png)


# Credits

# @KOEG

# Journal 4 60m

# Case

At first i imported the pcb into onshape as a step file with the 3d models so i could base the case off the 3d models.

![image](https://cdn.hackclub.com/01a0ee85-e0cd-77a5-91e0-4ae6dc409989/image.png)

Then i created a sketch that had a perimeter and some other features all included in the sketch so it would only take a few extrudes to finish it.

![image](https://cdn.hackclub.com/01a0ee85-e48b-7067-9239-61d1aaef8a3e/image.png)

Then i extruded the perimeter on the outside of the case.

![image](https://cdn.hackclub.com/01a0ee85-e866-7dd9-b176-b02df06c4085/image.png)

Now i created a top so it wouldn't be a open module.

![image](https://cdn.hackclub.com/01a0ee85-f4a3-7cd0-af2e-f8da0d3bf3c8/screenshot_from_2026-09-29_20-54-54.png)

Now it looked like this a bit not so nice, i wanted to create some fillits so it looked a bit nicer and here is a before and after.

![image](https://cdn.hackclub.com/01a0ee85-ef9c-712f-84e2-5dc9cb1b6a85/screenshot_from_2026-09-29_20-55-12.png)



![image](https://cdn.hackclub.com/01a0ee85-ec05-7fe5-82e3-10d5440f9695/screenshot_from_2026-09-29_20-55-15.png)

# Credits

# **@KOEG**