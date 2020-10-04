import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    application
    kotlin("jvm") version "1.4.10"
    id("org.jlleitschuh.gradle.ktlint") version "9.3.0"
}

application {
    mainClassName = "com.kylemayes.generator.MainKt"
}

group = "com.kylemayes"
version = "1.0.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "9"
}
