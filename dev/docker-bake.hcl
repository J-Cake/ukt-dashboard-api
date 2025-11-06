variable "builder_variant" {
	default = "hsb"
}

group "default" {
	targets = ["api", "kiosk"]
}

target "api" {
	dockerfile = "./dev/Dockerfile"
	args = {
		BUILD_VARIANT = "${builder_variant}"
	}
}

target "kiosk" {
	dockerfile = "./dev/Dockerfile"
	args = {
		BUILD_VARIANT = "${builder_variant}"
	}
}
