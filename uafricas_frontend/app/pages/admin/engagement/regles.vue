<script setup lang="ts">
import { ref, onMounted } from 'vue'
import {
  useAdminEngagement,
  type AdminRegle,
  type AdminPalier,
  type AdminNiveau,
} from '~/composables/useAdminEngagement'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  listerRegles, modifierRegle,
  listerPaliers, creerPalier, modifierPalier, desactiverPalier,
  listerNiveaux, modifierNiveau,
} = useAdminEngagement()

const regles = ref<AdminRegle[]>([])
const paliers = ref<AdminPalier[]>([])
const niveaux = ref<AdminNiveau[]>([])
const chargement = ref(true)
const message = ref('')

const nouveauPalier = ref({ seuil_likes: 0, points: 0 })

const rafraichir = async () => {
  chargement.value = true
  ;[regles.value, paliers.value, niveaux.value] = await Promise.all([
    listerRegles(), listerPaliers(), listerNiveaux(),
  ])
  chargement.value = false
}

onMounted(rafraichir)

const notifier = (m: string) => {
  message.value = m
  setTimeout(() => { message.value = '' }, 2500)
}

const enregistrerRegle = async (r: AdminRegle) => {
  await modifierRegle(r.id, {
    libelle: r.libelle,
    points: r.points,
    reputation_delta: r.reputation_delta,
    plafond_journalier: r.plafond_journalier,
    plafond_mensuel: r.plafond_mensuel,
    actif: r.actif,
  })
  notifier('Règle enregistrée')
}

const enregistrerPalier = async (p: AdminPalier) => {
  await modifierPalier(p.id, { points: p.points, actif: p.actif })
  notifier('Palier enregistré')
}

const ajouterPalier = async () => {
  if (nouveauPalier.value.seuil_likes <= 0) return
  await creerPalier(nouveauPalier.value.seuil_likes, nouveauPalier.value.points)
  nouveauPalier.value = { seuil_likes: 0, points: 0 }
  await rafraichir()
  notifier('Palier ajouté')
}

const retirerPalier = async (p: AdminPalier) => {
  await desactiverPalier(p.id)
  await rafraichir()
}

const enregistrerNiveau = async (n: AdminNiveau) => {
  await modifierNiveau(n.id, {
    libelle: n.libelle,
    seuil_min: n.seuil_min,
    badge_couleur: n.badge_couleur ?? undefined,
    badge_icone: n.badge_icone ?? undefined,
  })
  notifier('Niveau enregistré')
}
</script>

<template>
  <div class="p-6 space-y-8">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold">Engagement — Barème</h1>
      <NuxtLink to="/admin/engagement/journal" class="btn btn-sm btn-outline">
        <font-awesome-icon icon="fa-solid fa-list" /> Journal des points
      </NuxtLink>
    </div>

    <div v-if="message" class="alert alert-success py-2">{{ message }}</div>
    <div v-if="chargement" class="loading loading-spinner"></div>

    <template v-else>
      <!-- Règles -->
      <section>
        <h2 class="text-lg font-semibold mb-3">Règles de points</h2>
        <div class="overflow-x-auto">
          <table class="table table-zebra">
            <thead>
              <tr>
                <th>Action</th><th>Libellé</th><th>Points</th><th>Réputation</th>
                <th>Plafond / jour</th><th>Plafond / mois</th><th>Actif</th><th></th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="r in regles" :key="r.id">
                <td class="font-mono text-xs">{{ r.type_action }}</td>
                <td><input v-model="r.libelle" class="input input-sm input-bordered w-48" /></td>
                <td><input v-model.number="r.points" type="number" class="input input-sm input-bordered w-20" /></td>
                <td><input v-model.number="r.reputation_delta" type="number" class="input input-sm input-bordered w-20" /></td>
                <td><input v-model.number="r.plafond_journalier" type="number" class="input input-sm input-bordered w-24" placeholder="∞" /></td>
                <td><input v-model.number="r.plafond_mensuel" type="number" class="input input-sm input-bordered w-24" placeholder="∞" /></td>
                <td><input v-model="r.actif" type="checkbox" class="toggle toggle-sm" /></td>
                <td><button class="btn btn-xs btn-primary" @click="enregistrerRegle(r)">Enregistrer</button></td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- Paliers de popularité -->
      <section>
        <h2 class="text-lg font-semibold mb-3">Paliers de popularité (likes → points)</h2>
        <div class="overflow-x-auto">
          <table class="table table-zebra">
            <thead><tr><th>Seuil (likes)</th><th>Points</th><th>Actif</th><th></th></tr></thead>
            <tbody>
              <tr v-for="p in paliers" :key="p.id">
                <td>{{ p.seuil_likes }}</td>
                <td><input v-model.number="p.points" type="number" class="input input-sm input-bordered w-24" /></td>
                <td><input v-model="p.actif" type="checkbox" class="toggle toggle-sm" /></td>
                <td class="flex gap-2">
                  <button class="btn btn-xs btn-primary" @click="enregistrerPalier(p)">Enregistrer</button>
                  <button class="btn btn-xs btn-error btn-outline" @click="retirerPalier(p)">Désactiver</button>
                </td>
              </tr>
              <tr>
                <td><input v-model.number="nouveauPalier.seuil_likes" type="number" class="input input-sm input-bordered w-24" placeholder="ex. 2000" /></td>
                <td><input v-model.number="nouveauPalier.points" type="number" class="input input-sm input-bordered w-24" placeholder="ex. 80" /></td>
                <td></td>
                <td><button class="btn btn-xs btn-success" @click="ajouterPalier">Ajouter</button></td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- Niveaux -->
      <section>
        <h2 class="text-lg font-semibold mb-3">Seuils de niveaux</h2>
        <div class="overflow-x-auto">
          <table class="table table-zebra">
            <thead><tr><th>Code</th><th>Libellé</th><th>Seuil min</th><th>Badge</th><th></th></tr></thead>
            <tbody>
              <tr v-for="n in niveaux" :key="n.id">
                <td class="font-mono text-xs">{{ n.code }}</td>
                <td><input v-model="n.libelle" class="input input-sm input-bordered w-48" /></td>
                <td><input v-model.number="n.seuil_min" type="number" class="input input-sm input-bordered w-24" /></td>
                <td class="flex gap-1">
                  <input v-model="n.badge_couleur" class="input input-sm input-bordered w-24" placeholder="couleur" />
                  <input v-model="n.badge_icone" class="input input-sm input-bordered w-24" placeholder="icône" />
                </td>
                <td><button class="btn btn-xs btn-primary" @click="enregistrerNiveau(n)">Enregistrer</button></td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>
    </template>
  </div>
</template>
