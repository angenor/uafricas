<script setup lang="ts">
/**
 * Back-office — seuils de niveaux (daisyUI autorisé ici).
 *
 * Extrait de `regles.vue` parce que ces mutations ont un effet ensembliste :
 * chacune rebascule le niveau de tous les comptes concernés, en une seule
 * transaction serveur. Le nombre de comptes recalculés est affiché après chaque
 * geste — sans ce retour, l'administrateur ne verrait pas l'effet réel.
 */
import { ref, onMounted } from 'vue'
import { useAdminEngagement, type AdminNiveau } from '~/composables/useAdminEngagement'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { listerNiveaux, creerNiveau, modifierNiveau, supprimerNiveau } = useAdminEngagement()

const niveaux = ref<AdminNiveau[]>([])
const chargement = ref(true)
const message = ref('')
const erreur = ref('')

const formulaire = ref(false)
const nouveau = ref({ code: '', libelle: '', seuil_min: 0, badge_couleur: '', badge_icone: '' })

/** Jetons reconnus par `EngagementBadgeStatut`. */
const COULEURS = ['gray', 'green', 'amber', 'slate']

const rafraichir = async () => {
  chargement.value = true
  niveaux.value = await listerNiveaux()
  chargement.value = false
}

onMounted(rafraichir)

const notifier = (m: string) => {
  message.value = m
  erreur.value = ''
  setTimeout(() => { message.value = '' }, 5000)
}

const signaler = (e: unknown) => {
  message.value = ''
  const data = (e as { data?: { error?: string } })?.data
  erreur.value = data?.error || 'L\'opération a échoué.'
}

/** Formule le retour ensembliste : c'est l'information que l'écran doit donner. */
const notifierRecalcul = (action: string, comptes: number) =>
  notifier(
    comptes > 0
      ? `${action} — ${comptes} compte${comptes > 1 ? 's' : ''} ont changé de niveau.`
      : `${action} — aucun compte n'a changé de niveau.`,
  )

const ajouter = async () => {
  const f = nouveau.value
  if (!f.code.trim() || !f.libelle.trim()) {
    erreur.value = 'Le code et le libellé sont obligatoires.'
    return
  }
  try {
    const res = await creerNiveau({
      code: f.code.trim(),
      libelle: f.libelle.trim(),
      seuil_min: f.seuil_min,
      badge_couleur: f.badge_couleur || null,
      badge_icone: f.badge_icone.trim() || null,
    })
    formulaire.value = false
    nouveau.value = { code: '', libelle: '', seuil_min: 0, badge_couleur: '', badge_icone: '' }
    if (res) niveaux.value = res.niveaux
    else await rafraichir()
    notifierRecalcul('Niveau créé', res?.comptes_recalcules ?? 0)
  } catch (e) { signaler(e) }
}

const enregistrer = async (n: AdminNiveau) => {
  try {
    const res = await modifierNiveau(n.id, {
      libelle: n.libelle,
      seuil_min: n.seuil_min,
      badge_couleur: n.badge_couleur ?? undefined,
      badge_icone: n.badge_icone ?? undefined,
    })
    if (res) niveaux.value = res.niveaux
    notifierRecalcul('Niveau enregistré', res?.comptes_recalcules ?? 0)
  } catch (e) { signaler(e) }
}

const supprimer = async (n: AdminNiveau) => {
  if (n.seuil_min === 0) {
    erreur.value = 'Le niveau plancher (seuil 0) ne peut pas être retiré : il classe tous les membres sans points.'
    return
  }
  if (!confirm(
    `Retirer le niveau « ${n.libelle} » ?\n\nLes membres qui le portent retomberont immédiatement au niveau inférieur.`,
  )) return
  try {
    const res = await supprimerNiveau(n.id)
    if (res) niveaux.value = res.niveaux
    else await rafraichir()
    notifierRecalcul('Niveau retiré', res?.comptes_recalcules ?? 0)
  } catch (e) { signaler(e) }
}
</script>

<template>
  <div class="p-6 space-y-6">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <h1 class="text-2xl font-bold">Engagement — Niveaux</h1>
      <div class="flex flex-wrap gap-2">
        <NuxtLink to="/admin/engagement/regles" class="btn btn-sm btn-outline">
          <font-awesome-icon icon="fa-solid fa-sliders" /> Barème
        </NuxtLink>
        <button class="btn btn-sm btn-primary" @click="formulaire = !formulaire">
          <font-awesome-icon :icon="formulaire ? 'fa-solid fa-xmark' : 'fa-solid fa-plus'" />
          {{ formulaire ? 'Annuler' : 'Nouveau niveau' }}
        </button>
      </div>
    </div>

    <div class="alert alert-info py-2 text-sm">
      <font-awesome-icon icon="fa-solid fa-circle-info" />
      <span>
        Toute mutation de niveau rebascule <strong>immédiatement</strong> les comptes concernés :
        l'ordre d'affichage est recalculé d'après le seuil, et aucun membre ne reste sur un statut
        périmé. Le <strong>code</strong> d'un niveau est immuable, les comptes le référencent par valeur.
      </span>
    </div>

    <div v-if="message" class="alert alert-success py-2">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error py-2">{{ erreur }}</div>
    <div v-if="chargement" class="loading loading-spinner" />

    <template v-else>
      <div v-if="formulaire" class="card bg-base-200">
        <div class="card-body gap-3">
          <h2 class="font-semibold">Créer un niveau</h2>
          <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
            <label class="form-control">
              <span class="label-text text-xs">Code (immuable ensuite)</span>
              <input v-model="nouveau.code" class="input input-sm input-bordered font-mono" placeholder="ex. or">
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Libellé</span>
              <input v-model="nouveau.libelle" class="input input-sm input-bordered" placeholder="ex. Membre Or">
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Seuil minimal (points)</span>
              <input v-model.number="nouveau.seuil_min" type="number" class="input input-sm input-bordered">
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Couleur du badge</span>
              <select v-model="nouveau.badge_couleur" class="select select-sm select-bordered">
                <option value="">— par défaut —</option>
                <option v-for="c in COULEURS" :key="c" :value="c">{{ c }}</option>
              </select>
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Icône du badge (FontAwesome)</span>
              <input v-model="nouveau.badge_icone" class="input input-sm input-bordered font-mono" placeholder="ex. trophy">
            </label>
          </div>
          <div class="card-actions justify-end">
            <button class="btn btn-sm btn-success" @click="ajouter">Créer</button>
          </div>
        </div>
      </div>

      <div class="overflow-x-auto">
        <table class="table table-zebra table-sm">
          <thead>
            <tr><th>Ordre</th><th>Code</th><th>Libellé</th><th>Seuil min</th><th>Badge</th><th /></tr>
          </thead>
          <tbody>
            <tr v-for="n in niveaux" :key="n.id">
              <td class="text-center text-xs opacity-60">{{ n.ordre }}</td>
              <td class="font-mono text-xs">
                {{ n.code }}
                <span v-if="n.seuil_min === 0" class="badge badge-ghost badge-sm ml-1">plancher</span>
              </td>
              <td><input v-model="n.libelle" class="input input-sm input-bordered w-44"></td>
              <td><input v-model.number="n.seuil_min" type="number" class="input input-sm input-bordered w-24" :disabled="n.seuil_min === 0"></td>
              <td class="flex gap-1">
                <select v-model="n.badge_couleur" class="select select-sm select-bordered w-28">
                  <option :value="null">—</option>
                  <option v-for="c in COULEURS" :key="c" :value="c">{{ c }}</option>
                </select>
                <input v-model="n.badge_icone" class="input input-sm input-bordered w-28 font-mono" placeholder="icône">
              </td>
              <td class="flex gap-1">
                <button class="btn btn-xs btn-primary" @click="enregistrer(n)">Enregistrer</button>
                <button
                  class="btn btn-xs btn-error btn-outline"
                  :disabled="n.seuil_min === 0 || niveaux.length <= 1"
                  :title="n.seuil_min === 0 ? 'Le niveau plancher ne peut pas être retiré' : 'Retirer'"
                  @click="supprimer(n)"
                >
                  <font-awesome-icon icon="fa-solid fa-trash" />
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>
  </div>
</template>
