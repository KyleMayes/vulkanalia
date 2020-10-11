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
    implementation("com.github.ajalt.clikt:clikt:3.0.1")
    implementation("io.github.microutils:kotlin-logging-jvm:2.0.2")
    implementation("org.jetbrains:annotations:20.0.0")
    implementation("org.kohsuke:github-api:1.116")
    implementation("org.slf4j:slf4j-simple:1.7.29")
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "9"
}
