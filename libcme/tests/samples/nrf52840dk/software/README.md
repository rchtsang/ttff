





Zephyr samples
==============




---






![](./hello_world.svg)


Hello World
-----------


### A simple sample that prints "Hello World"



![](./shell_module.svg)


Shell Module
------------


### Zephyr shell interface demonstration



![](./philosophers.svg)


Philosophers
------------


### Solution to the Dining Philosophers problem



![](./tensorflow_lite_micro.svg)


TensorFlow Lite Micro
---------------------


### Sample application replicating sine function



![](./micropython.svg)


MicroPython
-----------


### MicroPython Zephyr port demonstration



![](./blinky.svg)


Blinky
------


### LED blinking using the Zephyr GPIO API



![](./hello_world_user.svg)


Hello World (user)
------------------


### Hello World from userspace



![](./synchronization.svg)


Synchronization
---------------


### Thread synchronization and timing sample



![](./lz4.svg)


LZ4
---


### LZ4 compression and decompression



![](./rust-app.svg)


Rust App
--------


### Rust API bindings and libstd



![](./kenning-zephyr-runtime-microtvm.svg)


Kenning microTVM
----------------


### Gesture recognition using Kenning Zephyr Runtime (microTVM)



![](./kenning-zephyr-runtime-tflitemicro.svg)


Kenning TFLite Micro
--------------------


### Gesture recognition using Kenning Zephyr Runtime (TFLite Micro)



![](./kenning-zephyr-runtime-iree.svg)


Kenning IREE
------------


### Gesture recognition using Kenning Zephyr Runtime (IREE)











Hello World
===========




---




A simple sample that prints â€œHello Worldâ€ to the console.



Run locally
-----------




You can run the Hello World demo on the nRF52840\-DK\-NRF52840 board by
following the instructions below. Assuming you have *Python 3* and *pip*
installed on your Linux machine, run the following commands to download Renode
and the prebuilt binaries for this demo, and then run the simulation in Renode
on your own machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 hello_world

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 hello\_world demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_hello_world.ipynb)


UART output
-----------






Trace
-----







Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Hello World executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/hello_world/hello_world.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Hello World demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/hello_world/hello_world-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Hello World demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/hello_world/hello_world-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Hello World demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/hello_world/hello_world.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Hello World demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/hello_world/hello_world.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Hello World demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/hello_world/hello_world.html)









Shell Module
============




---




This demo demonstrates the Zephyr shell submodule, which allows the user to use
a console interface to control the operating system.



Run locally
-----------




You can run the Shell Module demo on the nRF52840\-DK\-NRF52840 board by
following the instructions below. Assuming you have *Python 3* and *pip*
installed on your Linux machine, run the following commands to download Renode
and the prebuilt binaries for this demo, and then run the simulation in Renode
on your own machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 shell_module

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 shell\_module demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_shell_module.ipynb)


UART output
-----------






Trace
-----







Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Shell Module executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/shell_module/shell_module.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Shell Module demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/shell_module/shell_module-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Shell Module demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/shell_module/shell_module-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Shell Module demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/shell_module/shell_module.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Shell Module demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/shell_module/shell_module.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Shell Module demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/shell_module/shell_module.html)









Philosophers
============




---




An implementation of a solution to the Dining Philosophers problem which
demonstrates the usage of multiple preemptible and cooperative threads of
differing priorities, as well as dynamic mutexes and thread sleeping.



Run locally
-----------




You can run the Philosophers demo on the nRF52840\-DK\-NRF52840 board by
following the instructions below. Assuming you have *Python 3* and *pip*
installed on your Linux machine, run the following commands to download Renode
and the prebuilt binaries for this demo, and then run the simulation in Renode
on your own machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 philosophers

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 philosophers demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_philosophers.ipynb)


UART output
-----------






Trace
-----







Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Philosophers executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/philosophers/philosophers.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Philosophers demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/philosophers/philosophers-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Philosophers demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/philosophers/philosophers-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Philosophers demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/philosophers/philosophers.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Philosophers demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/philosophers/philosophers.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Philosophers demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/philosophers/philosophers.html)









TensorFlow Lite Micro
=====================




---




This sample TensorFlow application replicates a sine wave and demonstrates the
basics of using TensorFlow Lite Micro. The model included with the sample is
trained to replicate a sine function and generates x values to print alongside
the y values predicted by the model.



Run locally
-----------




You can run the TensorFlow Lite Micro demo on the nRF52840\-DK\-NRF52840 board
by following the instructions below. Assuming you have *Python 3* and *pip*
installed on your Linux machine, run the following commands to download Renode
and the prebuilt binaries for this demo, and then run the simulation in Renode
on your own machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 tensorflow_lite_micro

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 tensorflow\_lite\_micro demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_tensorflow_lite_micro.ipynb)


UART output
-----------






Trace
-----







Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the TensorFlow Lite Micro executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/tensorflow_lite_micro/tensorflow_lite_micro.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the TensorFlow Lite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/tensorflow_lite_micro/tensorflow_lite_micro-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the TensorFlow Lite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/tensorflow_lite_micro/tensorflow_lite_micro-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the TensorFlow Lite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/tensorflow_lite_micro/tensorflow_lite_micro.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the TensorFlow Lite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/tensorflow_lite_micro/tensorflow_lite_micro.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the TensorFlow Lite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/tensorflow_lite_micro/tensorflow_lite_micro.html)









MicroPython
===========




---




This demo demonstrates the MicroPython Zephyr port by performing arithmetic
operations, and by defining and calling simple functions.



Run locally
-----------




You can run the MicroPython demo on the nRF52840\-DK\-NRF52840 board by
following the instructions below. Assuming you have *Python 3* and *pip*
installed on your Linux machine, run the following commands to download Renode
and the prebuilt binaries for this demo, and then run the simulation in Renode
on your own machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 micropython

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 micropython demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_micropython.ipynb)


UART output
-----------






Trace
-----







Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the MicroPython executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/micropython/micropython.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the MicroPython demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/micropython/micropython-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the MicroPython demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/micropython/micropython-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the MicroPython demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/micropython/micropython.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the MicroPython demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/micropython/micropython.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the MicroPython demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/micropython/micropython.html)









Blinky
======




---




A basic sample that blinks an LED forever using the Zephyr GPIO API.



Run locally
-----------







The Blinky demo is not supported in the Renode Zephyr dashboard on nRF52840\-DK\-NRF52840 yet.
----------------------------------------------------------------------------------------------


### You can contact us by clicking the button below if you want to see it supported.






[![](./mail.svg)


Contact us
----------


###](mailto:contact@antmicro.com)



Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Blinky executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/blinky/blinky.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Blinky demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/blinky/blinky-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Blinky demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/blinky/blinky-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Blinky demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/blinky/blinky.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Blinky demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/blinky/blinky.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Blinky demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/blinky/blinky.html)









Hello World (user)
==================




---




A sample that prints â€œHello Worldâ€ to the console from a usermode thread.



Run locally
-----------




You can run the Hello World (user) demo on the nRF52840\-DK\-NRF52840 board by
following the instructions below. Assuming you have *Python 3* and *pip*
installed on your Linux machine, run the following commands to download Renode
and the prebuilt binaries for this demo, and then run the simulation in Renode
on your own machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 hello_world_user

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 hello\_world\_user demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_hello_world_user.ipynb)


UART output
-----------






Trace
-----







Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Hello World (user) executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/hello_world_user/hello_world_user.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Hello World (user) demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/hello_world_user/hello_world_user-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Hello World (user) demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/hello_world_user/hello_world_user-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Hello World (user) demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/hello_world_user/hello_world_user.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Hello World (user) demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/hello_world_user/hello_world_user.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Hello World (user) demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/hello_world_user/hello_world_user.html)









Synchronization
===============




---




A simple application that demonstrates kernel scheduling, communication, and
timing. Two threads take turns printing â€œHello Worldâ€ synchronized by
semaphores.



Run locally
-----------




You can run the Synchronization demo on the nRF52840\-DK\-NRF52840 board by
following the instructions below. Assuming you have *Python 3* and *pip*
installed on your Linux machine, run the following commands to download Renode
and the prebuilt binaries for this demo, and then run the simulation in Renode
on your own machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 synchronization

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 synchronization demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_synchronization.ipynb)


UART output
-----------






Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Synchronization executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/synchronization/synchronization.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Synchronization demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/synchronization/synchronization-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Synchronization demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/synchronization/synchronization-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Synchronization demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/synchronization/synchronization.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Synchronization demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/synchronization/synchronization.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Synchronization demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/synchronization/synchronization.html)









LZ4
===




---




This sample application compresses and decompresses a block of data with the LZ4
algorithm and verifies the result.



Run locally
-----------




You can run the LZ4 demo on the nRF52840\-DK\-NRF52840 board by following the
instructions below. Assuming you have *Python 3* and *pip* installed on your
Linux machine, run the following commands to download Renode and the prebuilt
binaries for this demo, and then run the simulation in Renode on your own
machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 lz4

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 lz4 demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_lz4.ipynb)


UART output
-----------






Trace
-----







Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the LZ4 executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/lz4/lz4.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the LZ4 demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/lz4/lz4-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the LZ4 demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/lz4/lz4-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the LZ4 demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/lz4/lz4.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the LZ4 demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/lz4/lz4.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the LZ4 demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/lz4/lz4.html)









Rust App
========




---




This sample demonstrates integration of Rust code into Zephyr RTOS projects.



Run locally
-----------




You can run the Rust App demo on the nRF52840\-DK\-NRF52840 board by following
the instructions below. Assuming you have *Python 3* and *pip* installed on your
Linux machine, run the following commands to download Renode and the prebuilt
binaries for this demo, and then run the simulation in Renode on your own
machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 rust-app

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 rust\-app demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_rust-app.ipynb)


UART output
-----------






Trace
-----







Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Rust App executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/rust-app/rust-app.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Rust App demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/rust-app/rust-app-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Rust App demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/rust-app/rust-app-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Rust App demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/rust-app/rust-app.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Rust App demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/rust-app/rust-app.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Rust App demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/rust-app/rust-app.html)









Kenning microTVM
================




---




This is a demonstration application of Kenning Zephyr Runtime running gesture
recognition model on sample data using microTVM.



Run locally
-----------




You can run the Kenning microTVM demo on the nRF52840\-DK\-NRF52840 board by
following the instructions below. Assuming you have *Python 3* and *pip*
installed on your Linux machine, run the following commands to download Renode
and the prebuilt binaries for this demo, and then run the simulation in Renode
on your own machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 kenning-zephyr-runtime-microtvm

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 kenning\-zephyr\-runtime\-microtvm demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_kenning-zephyr-runtime-microtvm.ipynb)


UART output
-----------






Trace
-----







Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Kenning microTVM executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/kenning-zephyr-runtime-microtvm/kenning-zephyr-runtime-microtvm.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Kenning microTVM demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/kenning-zephyr-runtime-microtvm/kenning-zephyr-runtime-microtvm-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Kenning microTVM demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-microtvm/kenning-zephyr-runtime-microtvm-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Kenning microTVM demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-microtvm/kenning-zephyr-runtime-microtvm.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Kenning microTVM demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-microtvm/kenning-zephyr-runtime-microtvm.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Kenning microTVM demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-microtvm/kenning-zephyr-runtime-microtvm.html)









Kenning TFLite Micro
====================




---




This is a demonstration application of Kenning Zephyr Runtime running gesture
recognition model on sample data using TFLite Micro.



Run locally
-----------




You can run the Kenning TFLite Micro demo on the nRF52840\-DK\-NRF52840 board by
following the instructions below. Assuming you have *Python 3* and *pip*
installed on your Linux machine, run the following commands to download Renode
and the prebuilt binaries for this demo, and then run the simulation in Renode
on your own machine:



```
pip3 install --user --upgrade git+https://github.com/antmicro/renode-run.git
renode-run demo -b nrf52840dk_nrf52840 kenning-zephyr-runtime-tflitemicro

```


Run in Colab
------------



You can run this demo instantly on a cloud server in Google Colab by clicking
the button below.



[![](./colab.svg)


Colab
-----


### Run nrf52840dk\_nrf52840 kenning\-zephyr\-runtime\-tflitemicro demo in Google Colab.](https://colab.research.google.com/github/antmicro/test-colabs/blob/main/boards/nrf52840dk_nrf52840_kenning-zephyr-runtime-tflitemicro.ipynb)


UART output
-----------






Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Kenning TFLite Micro executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/kenning-zephyr-runtime-tflitemicro/kenning-zephyr-runtime-tflitemicro.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Kenning TFLite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/kenning-zephyr-runtime-tflitemicro/kenning-zephyr-runtime-tflitemicro-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Kenning TFLite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-tflitemicro/kenning-zephyr-runtime-tflitemicro-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Kenning TFLite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-tflitemicro/kenning-zephyr-runtime-tflitemicro.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Kenning TFLite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-tflitemicro/kenning-zephyr-runtime-tflitemicro.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Kenning TFLite Micro demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-tflitemicro/kenning-zephyr-runtime-tflitemicro.html)









Kenning IREE
============




---




This is a demonstration application of Kenning Zephyr Runtime running gesture
recognition model on sample data using IREE.



Run locally
-----------







The Kenning IREE demo is not supported in the Renode Zephyr dashboard on nRF52840\-DK\-NRF52840 yet.
----------------------------------------------------------------------------------------------------


### You can contact us by clicking the button below if you want to see it supported.






[![](./mail.svg)


Contact us
----------


###](mailto:contact@antmicro.com)



Download
--------




[![](./artifacts.svg)


Zephyr binary
-------------


### Download the Kenning IREE executable file




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/kenning-zephyr-runtime-iree/kenning-zephyr-runtime-iree.elf)


[![](./sbom.svg)


SBOM data
---------


### Download Software Bill of Materials data for the Kenning IREE demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr/cce7e9a706ca848f73e4ef2c435668e358b73305/nrf52840dk_nrf52840/kenning-zephyr-runtime-iree/kenning-zephyr-runtime-iree-sbom.zip)


[![](./renode-artifacts.svg)


Renode log
----------


### Download the Renode simulation log for the Kenning IREE demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-iree/kenning-zephyr-runtime-iree-renode.log)


[![](./renode-artifacts.svg)


Renode script
-------------


### Download the Renode script for the Kenning IREE demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-iree/kenning-zephyr-runtime-iree.resc)


[![](./renode-artifacts.svg)


Renode Robot test suite
-----------------------


### Download the Renode Robot test suite for the Kenning IREE demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-iree/kenning-zephyr-runtime-iree.robot)


[![](./robot.svg)


Robot test suite log
--------------------


### See Robot test results for the Kenning IREE demo




![](./download.svg)](https://zephyr-dashboard.renode.io/zephyr_sim/cce7e9a706ca848f73e4ef2c435668e358b73305/914fdfa5f16c93ef6598a47c977bfaafb88c4555/nrf52840dk_nrf52840/kenning-zephyr-runtime-iree/kenning-zephyr-runtime-iree.html)





