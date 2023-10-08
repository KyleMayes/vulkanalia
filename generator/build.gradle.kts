
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    application
    kotlin("jvm") version "1.9.10"
    id("org.jlleitschuh.gradle.ktlint") version "11.6.0"
}

application {
    mainClass.set("com.kylemayes.generator.MainKt")
}

group = "com.kylemayes"
version = "1.0.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("com.github.ajalt.clikt:clikt:4.2.0")
    implementation("com.vladsch.flexmark:flexmark-all:0.64.8")
    implementation("io.github.microutils:kotlin-logging-jvm:3.0.5")
    implementation("org.jetbrains:annotations:24.0.1")
    implementation("org.kohsuke:github-api:1.316")
    implementation("org.slf4j:slf4j-simple:2.0.9")
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.0")
    testImplementation("org.mockito:mockito-core:5.6.0")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

java {
    sourceCompatibility = JavaVersion.VERSION_13
    targetCompatibility = JavaVersion.VERSION_13
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "13"
}

tasks.named<Test>("test") {
    useJUnitPlatform()
}
