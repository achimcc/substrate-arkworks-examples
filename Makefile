################################################################################
#
# You can run them like `make install` or `make benchmark`
# 
################################################################################




################################################################################
# INSTALL
################################################################################
install-jq:
ifeq ($(shell uname), Linux)
@echo "Linux"
endif
	# ifeq ($(shell uname), Linux)
	
	# ifeq ($(shell lsb_release -si), ManjaroLinux)
	# @echo "Manjaro"
	# yes | sudo pacman -S jq
	# else ifeq ($(shell lsb_release -si), ArchLinux)
	# yes | sudo pacman -S jq
	# else
	# sudo apt-get install jq -y
	# else
	# @echo "Fail"
	# endif 
	# endif
install-csv-to-markdown:
	sudo npm install --global csv-to-markdown

install: install-jq
instal: install-csv-to-markdown
# install:
	# ifeq $(shell uname), Linux
	# 	sudo apt install pandoc 
	# else
	# 	brew install pandoc
	# endif


################################################################################
# BENCHMARK
################################################################################



    

